use anyhow::{anyhow, Result};
use tauri_plugin_rrai_idns::task::tasks_worker_wakeup;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    tracing::info!("RRAI 启动...");

    //
    let token = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyaWQiOiI1OTEyMDg4ZTQ4YmM0MGFjODQ2MjJiMzMwZTg0MTc1ZiIsInVuaW9uaWQiOiJvbXVSV3c4N2w1Q3ZacVhaZmFTbmVzRnVXZVBvIiwib3BlbmlkIjoib2VDTEo1a0FkM0d5RnN2bDhRWWF0WTZIS3RzRSIsImV4cCI6MTY4OTE1MTg5MzM4Mn0.oHfz3HhnUBB5w-RAo62wkcIPmaiw8fyB5DGzs2eN8mY");

    let res = tasks_worker_wakeup(&token).await;

    match res {
        Ok(_) => {}
        Err(err) => {
            println!("{:#?}", err);
        }
    }
    Ok(())
}
