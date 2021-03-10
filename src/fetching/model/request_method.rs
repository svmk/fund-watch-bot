
#[derive(Debug, Clone)]
pub enum RequestMethod {
    Get,
    Post,
}

impl RequestMethod {
    pub fn has_body(&self) -> bool {
        match self {
            RequestMethod::Get => {
                return false;
            },
            RequestMethod::Post => {
                return true;
            },
        }
    }
}