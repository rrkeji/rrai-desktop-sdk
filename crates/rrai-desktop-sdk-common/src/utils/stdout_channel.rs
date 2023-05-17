pub use super::rate_limiter::RateLimiter;

use deadqueue::unlimited::Queue;
use std::{fmt, fmt::Display, io::Write, ops::Deref, sync::Arc};
use tokio::{
    io::{stderr, stdout, AsyncWriteExt},
    sync::Mutex,
    task::{spawn, JoinHandle},
};
use thiserror::Error;
use tokio::task::JoinError;
use std::io::Error as IoError;

#[derive(Error, Debug)]
pub enum StdoutChannelError {
    #[error("task join error")]
    JoinError(#[from] JoinError),
    #[error("io error")]
    IoError(#[from] IoError),
}

enum StdoutMessage<T> {
    Mesg(T),
    Close,
}

type StdoutQueue<T> = Queue<StdoutMessage<T>>;
type StdoutTask = JoinHandle<Result<(), StdoutChannelError>>;

#[derive(Clone)]
pub struct StdoutChannel<T> {
    stdout_queue: Arc<StdoutQueue<T>>,
    stderr_queue: Arc<StdoutQueue<T>>,
    stdout_task: Arc<Mutex<Option<StdoutTask>>>,
    stderr_task: Arc<Mutex<Option<StdoutTask>>>,
}

impl<T> Default for StdoutChannel<T>
where
    T: Display + Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Debug for StdoutChannel<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StdoutChannel")
    }
}

impl<T> StdoutChannel<T>
where
    T: Display + Send + 'static,
{
    #[must_use]
    pub fn new() -> Self {
        let stdout_queue = Queue::new().into();
        let stderr_queue = Queue::new().into();
        let stdout_task = Mutex::new(Some(spawn({
            let queue = Arc::clone(&stdout_queue);
            async move { Self::process_stdout(&queue).await }
        })))
        .into();
        let stderr_task = Mutex::new(Some(spawn({
            let queue = Arc::clone(&stderr_queue);
            async move { Self::process_stderr(&queue).await }
        })))
        .into();
        Self {
            stdout_queue,
            stderr_queue,
            stdout_task,
            stderr_task,
        }
    }

    #[must_use]
    pub fn with_mock_stdout(mock_stdout: MockStdout<T>, mock_stderr: MockStdout<T>) -> Self {
        let stdout_queue = Queue::new().into();
        let stderr_queue = Queue::new().into();
        let stdout_task = Mutex::new(Some(spawn({
            let queue = Arc::clone(&stdout_queue);
            async move { Self::process_mock(&queue, &mock_stdout).await }
        })))
        .into();
        let stderr_task = Mutex::new(Some(spawn({
            let queue = Arc::clone(&stderr_queue);
            async move { Self::process_mock(&queue, &mock_stderr).await }
        })))
        .into();
        Self {
            stdout_queue,
            stderr_queue,
            stdout_task,
            stderr_task,
        }
    }

    pub fn send(&self, item: impl Into<T>) {
        self.stdout_queue.push(StdoutMessage::Mesg(item.into()));
    }

    pub fn send_err(&self, item: impl Into<T>) {
        self.stderr_queue.push(StdoutMessage::Mesg(item.into()));
    }

    /// Close the `StdoutChannel`
    /// # Errors
    ///
    /// Will error if there have been any errors or panics in the stdout and
    /// stderr tasks
    pub async fn close(&self) -> Result<(), StdoutChannelError> {
        self.stdout_queue.push(StdoutMessage::Close);
        self.stderr_queue.push(StdoutMessage::Close);
        if let Some(stdout_task) = self.stdout_task.lock().await.take() {
            stdout_task.await??;
        }
        if let Some(stderr_task) = self.stderr_task.lock().await.take() {
            stderr_task.await??;
        }
        Ok(())
    }

    async fn process_stdout(queue: &StdoutQueue<T>) -> Result<(), StdoutChannelError> {
        let mut buf = Buffer::new();
        while let StdoutMessage::Mesg(line) = queue.pop().await {
            stdout().write_all(buf.write_line(line)?).await?;
        }
        Ok(())
    }

    async fn process_stderr(queue: &StdoutQueue<T>) -> Result<(), StdoutChannelError> {
        let mut buf = Buffer::new();
        while let StdoutMessage::Mesg(line) = queue.pop().await {
            stderr().write_all(buf.write_line(line)?).await?;
        }
        Ok(())
    }

    async fn process_mock(
        queue: &StdoutQueue<T>,
        mock_stdout: &MockStdout<T>,
    ) -> Result<(), StdoutChannelError> {
        while let StdoutMessage::Mesg(line) = queue.pop().await {
            mock_stdout.lock().await.push(line);
        }
        Ok(())
    }
}

const MAX_BUFFER_CAPACITY: usize = 4096;

struct Buffer(Vec<u8>);

impl Buffer {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn write_line<T: Display>(&mut self, line: T) -> Result<&[u8], StdoutChannelError> {
        self.0.clear();
        if self.0.capacity() > MAX_BUFFER_CAPACITY {
            self.0.shrink_to(MAX_BUFFER_CAPACITY);
        }
        writeln!(self.0, "{}", line)?;
        Ok(&self.0)
    }
}

#[derive(Clone)]
pub struct MockStdout<T>(Arc<Mutex<Vec<T>>>);

impl<T> Default for MockStdout<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for MockStdout<T> {
    type Target = Mutex<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MockStdout<T> {
    #[must_use]
    pub fn new() -> Self {
        Self(Mutex::new(Vec::new()).into())
    }
}
