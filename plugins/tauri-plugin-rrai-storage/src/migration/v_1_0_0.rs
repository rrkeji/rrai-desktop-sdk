use rrai_desktop_sdk_common::sqlite::migration::NormalDdlDatabaseVersionSql;

pub fn v_1_0_0() -> NormalDdlDatabaseVersionSql {
    NormalDdlDatabaseVersionSql::new(1, vec![include_str!("v_1_0_0.sql").to_string()])
}
