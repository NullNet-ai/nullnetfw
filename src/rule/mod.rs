pub mod verdict;

// @TODO
pub enum MatchExpr {}

// @TODO
pub enum Statement {}

pub struct Rule {
    matchexpt: MatchExpr,
    statement: Statement,
    handle: Option<i32>,
}
