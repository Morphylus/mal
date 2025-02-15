#[derive(Clone, Debug)]
pub enum Mal {
    List(Vec<Mal>),
    Int(i32),
    Sym(String),
}
