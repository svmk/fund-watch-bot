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