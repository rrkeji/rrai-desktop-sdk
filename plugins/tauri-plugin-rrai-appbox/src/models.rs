use serde::{Deserialize, Serialize};

/// 定义 User 类型
#[derive(Debug, Serialize, Deserialize)]
struct RraiApplication {
    pub id: String,
    pub provider: String,
    pub application_cid: String,
    pub application_type: String,
    pub name: String,
    pub avatar: String,
    pub labels: Vec<String>,
    pub signature: String,
    pub description: String,
    pub create_at: u64,
}
