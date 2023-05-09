use crate::utils::execute_command;
use anyhow::{anyhow, Result};

pub async fn is_available() -> Result<(bool, String)> {
    //
    if let (Some(code), message) = execute_command(&String::from("python3 --version")).await? {
        if code == 0 {
            Ok((true, message))
        } else {
            Ok((false, message))
        }
    } else {
        Err(anyhow!(""))
    }
}
