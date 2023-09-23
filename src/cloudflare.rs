use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct APIError {
    pub code: u16,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct BaseResponse {
    pub success: bool,
    pub errors: Option<Vec<APIError>>,
}
