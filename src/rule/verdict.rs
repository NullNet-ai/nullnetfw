#[derive(Debug, Clone)]
pub enum Verdict {
    Accept,
    Drop,
    Queue,
    Continue,
    Return,
    Jump(String),
    Goto(String),
}
