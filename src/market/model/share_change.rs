use crate::market::model::share::Share;

#[derive(Debug, Clone)]
pub struct ShareChange {
    from: Share,
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