use anyhow::{anyhow, Result};
use bytes::{BufMut, BytesMut};
use futures::stream::StreamExt;
use std::io::Cursor;
use tokio::runtime::Handle;

use super::get_ipfs_client;
use crate::ipfs_api::{IpfsApi, IpfsClient};

/// 存储值，并返回内容ID
pub async fn ipfs_add_content(value: Vec<u8>) -> Result<String> {
    //
    //保存到到IPFS
    let data = Cursor::new(value);
    match get_ipfs_client() {
        Ok(client) => {
            let res_result = client.add(data).await;
            match res_result {
                Ok(res) => {
                    tracing::debug!("保存IPFS成功:{:#?}", res);
                    Ok(res.hash)
                }
                Err(e) => {
                    tracing::error!("保存到IPFS失败:{:#?}", e);
                    Err(anyhow!(""))
                }
            }
        }
        _ => Err(anyhow!("")),
    }
}

/// 存储值，并返回内容ID
pub fn _ipfs_add_content(value: Vec<u8>) -> Result<String> {
    //
    //保存到到IPFS
    let data = Cursor::new(value);

    let handle = Handle::current();
    let handle_std = std::thread::spawn(move || {
        handle.block_on(async move {
            if let Ok(client) = get_ipfs_client() {
                let res_result = client.add(data).await;
                match res_result {
                    Ok(res) => {
                        tracing::debug!("保存IPFS成功:{:#?}", res);
                        Ok(res.hash)
                    }
                    Err(e) => {
                        tracing::error!("保存到IPFS失败:{:#?}", e);
                        Err(anyhow!(""))
                    }
                }
            } else {
                Err(anyhow!(""))
            }
        })
    });

    if let Ok(res) = handle_std.join() {
        res
    } else {
        Err(anyhow!(""))
    }
}
