use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseResult<T> {
    #[serde(rename="result")]
    result: Option<T>,
    #[serde(rename="error")]
    error: Option<String>,
}

impl <T>ResponseResult<T> {
    pub fn get_result(&self) -> Result<&T, Failure> {
        match (&self.result, &self.error) {
            (Some(_result), Some(error)) => {
                return crate::fail!("Yahoo error: {}", error);
            },
            (None, Some(error)) => {
                return crate::fail!("Yahoo error: {}", error);
            },
            (Some(result), None) => {
                return Ok(result);
            },
            (None, None) => {
                return crate::fail!("Unknown yahoo error");
            },
        }
    }
}