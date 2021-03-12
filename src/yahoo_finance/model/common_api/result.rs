#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseResult<T> {
    Ok {
        #[serde(rename="result")]
        result: T,
    },
    Error {
        #[serde(rename="error")]
        error: String,
    },
}