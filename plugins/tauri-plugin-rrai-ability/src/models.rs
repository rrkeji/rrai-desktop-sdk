use serde::{Deserialize, Serialize};

/// 定义 FileEntity 类型
#[derive(Debug, Serialize, Deserialize)]
pub struct AbilityEntity {
    pub id: u32,
    pub is_available: u32,
    pub ability: String,
    pub version: String,
    pub icon: String,
    pub dependencies: String,
    pub category: String,
    pub settings: String,
    pub created_at: u64,
    pub updated_at: u64,
}
