use serde::{Deserialize, Serialize};

/// 定义 AbilityEntity 类型
#[derive(Debug, Serialize, Deserialize)]
pub struct AbilityEntity {
    pub id: u32,
    pub is_available: u32,
    pub ability: String,
    pub ability_name: String,
    pub version: String,
    pub version_infor: String,
    pub icon: String,
    pub dependencies: String,
    pub category: String,
    pub settings_schema: String,
    pub install_guide: String,
    pub settings: String,
    pub created_at: u64,
    pub updated_at: u64,
}
 
/// 定义 LocalTaskEntity 类型
#[derive(Debug, Serialize, Deserialize)]
pub struct LocalTaskEntity {
    pub id: u32,
    pub task_id: String,
    pub ability: String,
    pub args: String,
    pub remote: u16,
    pub remote_task_id: String,
    pub remote_server: String,
    pub result_code:u16,
    pub stdout: String,
    pub stderr: String,
    pub result: String,
    pub created_at: u64,
    pub updated_at: u64,
}
