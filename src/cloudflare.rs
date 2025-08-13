use color_eyre::eyre::{eyre, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CloudflareError {
    pub code: u16,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct CloudflareResponse<T> {
    pub errors: Option<Vec<CloudflareError>>,
    pub result: Option<T>,
}

impl<T> CloudflareResponse<T> {
    /// Handle the Cloudflare error by wrapping it in a Result
    pub fn cf_wrap(self, error_hint: &str) -> Result<Option<T>> {
        match self.errors {
            None => Ok(self.result),
            Some(errors) => {
                if errors.len() == 0 {
                    return Ok(self.result);
                }

                for error in errors {
                    println!("  ERROR({}): {}", error.code, error.message);
                }

                Err(eyre!(error_hint.to_owned()))
            }
        }
    }
}
