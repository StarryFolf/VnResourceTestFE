use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResponseResult<T> {
    succeeded: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}