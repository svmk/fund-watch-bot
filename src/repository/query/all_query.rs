use crate::repository::model::query::Query;
#[derive(Debug)]
pub struct AllQuery {

}

impl Query for AllQuery {}

impl AllQuery {
    pub fn new() -> AllQuery {
        return AllQuery {};
    }
}