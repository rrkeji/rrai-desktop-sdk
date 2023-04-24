use serde::{Deserialize, Serialize};

/// 定义 User 类型
#[derive(Debug, Serialize, Deserialize)]
struct RraiApplication {
    name: String,
    age: i32,
    gender: String,
    friends: Vec<String>,
}
