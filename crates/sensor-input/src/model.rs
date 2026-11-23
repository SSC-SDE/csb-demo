#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reading {
    pub sensor: String,
    pub timestamp: u64,
    pub value: i64,
}
