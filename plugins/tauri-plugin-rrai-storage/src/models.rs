use serde::{Deserialize, Serialize};

/// 定义 FileEntity 类型
#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntity {
    pub id: u32,
    pub parent_id: u32,
    pub cid: String,
    pub is_pin: bool,
    pub file_name: String,
    pub file_hash: String,
    pub file_type: String,
    pub category: String,
    pub avatar: String,
    pub is_dir: bool,
    pub created_at: u64,
    pub updated_at: u64,
}
