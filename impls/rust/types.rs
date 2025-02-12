#[derive(Clone)]
pub enum MalType {
    MalList(Vec<MalType>),
    MalInteger(i32),
    MalSymbol(String),
}
