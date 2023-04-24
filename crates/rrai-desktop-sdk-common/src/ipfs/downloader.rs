use crate::ipfs_api::IpfsApi;
use anyhow::Result;
use bytes::{BufMut, BytesMut};
use futures::stream::StreamExt;
use std::io::Cursor;

use super::get_ipfs_client;

pub async fn ipfs_get_content(cid: &String) -> Result<Vec<u8>> {
    //
    let client = get_ipfs_client()?;
    //读取数据
    let mut stream = client.cat(cid.as_str());
    //
    let mut buf = BytesMut::with_capacity(40960);

    while let Some(parts) = stream.next().await {
        // bytes.
        if let Ok(bs) = parts {
            buf.put(bs);
        }
    }
    Ok(buf.to_vec())
}
