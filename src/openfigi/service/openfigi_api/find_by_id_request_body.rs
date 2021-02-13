use crate::market::model::cusip::Cusip;

#[derive(Serialize, Deserialize)]
pub struct FindByIdRequestBody {
    #[serde(rename = "idType")]
    id_type: String,
    #[serde(rename = "idValue")]
    id_value: String,
}

impl FindByIdRequestBody {
    const REQUEST_CUSIP: &'static str = "ID_CUSIP";

    pub fn new_request_cusip(value: Cusip) -> FindByIdRequestBody {
        return FindByIdRequestBody {
            id_type: Self::REQUEST_CUSIP.to_string(),
            id_value: value.into_to_string(),
        };
    }
}