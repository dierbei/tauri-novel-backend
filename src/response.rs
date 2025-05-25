use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SuccessResponse<T> {
    pub code: u32,
    pub data: T,
    pub msg: String,
}

pub fn success<T>(data: T) -> SuccessResponse<T> {
    SuccessResponse {
        code: 200,
        data,
        msg: "success".to_string(),
    }
}