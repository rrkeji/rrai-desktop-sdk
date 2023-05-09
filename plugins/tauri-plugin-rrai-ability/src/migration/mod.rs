use anyhow::Result;
use rrai_desktop_sdk_common::sqlite::migration::{
    merge_database_version, DatabaseVersionSql, NormalDdlDatabaseVersionSql,
};
mod v_1_0_0;

pub async fn init_database() -> Result<()> {
    let list: Vec<NormalDdlDatabaseVersionSql> = vec![v_1_0_0::v_1_0_0()];

    // tracing::debug!("init_database:{:#?}", list);
    tracing::debug!("init_database files");
    let _ = merge_database_version(&crate::constants::ABILITIES_DATABASE_NAME.to_string(), list)
        .await?;

    Ok(())
}
