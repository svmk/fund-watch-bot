use crate::market::common::model::cusip::Cusip;

#[derive(Serialize, Deserialize)]
pub struct FindByIdRequestBody {
    #[serde(rename = "idType")]
    id_type: String,
    #[serde(rename = "idValue")]
    id_value: String,
}

impl FindByIdRequestBody {
    const REQUEST_CUSIP: &'static str = "ID_CUSIP";
    const REQUEST_CINS: &'static str = "ID_CINS";

    pub fn new_request_cusip(value: Cusip) -> FindByIdRequestBody {
        return FindByIdRequestBody {
            id_type: Self::REQUEST_CUSIP.to_string(),
            id_value: value.into_to_string(),
        };
    }

    pub fn new_request_cins(value: Cusip) -> FindByIdRequestBody {
        return FindByIdRequestBody {
            id_type: Self::REQUEST_CINS.to_string(),
            id_value: value.into_to_string(),
        };
    }
}