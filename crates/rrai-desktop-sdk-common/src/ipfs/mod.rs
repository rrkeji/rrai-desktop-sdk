mod downloader;
mod uploader;

pub use downloader::*;
pub use uploader::*;

use crate::ipfs_api::IpfsClient;
use anyhow::Result;

pub fn get_ipfs_client() -> Result<IpfsClient> {
    //
    let res = IpfsClient::default();

    Ok(res)
}
