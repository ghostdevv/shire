use color_eyre::eyre::{eyre, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct APIError {
    pub code: u16,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct BaseResponse {
    pub errors: Option<Vec<APIError>>,
}

pub fn assert_cf_errors(errors: &Option<Vec<APIError>>, message: String) -> Result<()> {
    match errors {
        None => Ok(()),
        Some(errors) => {
            if errors.len() == 0 {
                return Ok(());
            }

            for error in errors {
                println!("  ERROR({}): {}", error.code, error.message);
            }

            Err(eyre!(message))
        }
    }
}
