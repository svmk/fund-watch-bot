#[derive(Debug)]
pub struct Year(u16);

impl Year {
    pub fn new(year: u16) -> Year {
        return Year(year);
    }
}