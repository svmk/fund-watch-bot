#[derive(Debug)]
pub struct Quartal(u8);

impl Quartal {
    pub const Q1: Quartal = Quartal(1);
    pub const Q2: Quartal = Quartal(2);
    pub const Q3: Quartal = Quartal(3);
    pub const Q4: Quartal = Quartal(4);
}