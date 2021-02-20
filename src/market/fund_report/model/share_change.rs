use crate::market::common::model::share::Share;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareChange {
    #[serde(rename = "from")]
    from: Share,
    #[serde(rename = "to")]
    to: Share,
}

impl ShareChange {
    pub fn new(
        from: Share,
        to: Share,
    ) -> Option<ShareChange> {
        if from == to {
            return None;
        }
        return Some(ShareChange {
            from,
            to,
        });
    }
}