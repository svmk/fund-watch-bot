use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Data {
        #[serde(rename="data")]
        data: T,
    },
    Error {
        #[serde(rename="error")]
        error: String,
    },
}

impl <T>Response<T> {
    pub fn into_result(self) -> Result<T, Failure> {
        match self {
            Response::Data { data } => {
                return Ok(data);
            },
            Response::Error { error } => {
                return crate::fail!("Openfigi error: {}", error);
            },
        }
    } 
}