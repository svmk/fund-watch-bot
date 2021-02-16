use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum Quartal {
    #[serde(rename = "Q1")]
    Q1,
    #[serde(rename = "Q2")]
    Q2,
    #[serde(rename = "Q3")]
    Q3,
    #[serde(rename = "Q4")]
    Q4,
}

impl Quartal {
    pub fn display_long(&self) -> &str {
        match self {
            Quartal::Q1 => {
                return "QTR1";
            },
            Quartal::Q2 => {
                return "QTR2";
            },
            Quartal::Q3 => {
                return "QTR3";
            },
            Quartal::Q4 => {
                return "QTR4";
            },
        }
    }
}


impl fmt::Display for Quartal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quartal::Q1 => {
                write!(f, "Q1")
            },
            Quartal::Q2 => {
                write!(f, "Q2")
            },
            Quartal::Q3 => {
                write!(f, "Q3")
            },
            Quartal::Q4 => {
                write!(f, "Q4")
            },
        }
    }
}