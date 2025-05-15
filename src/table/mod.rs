mod family;

pub use family::Family;

#[derive(Debug, Clone)]
pub struct Table {
    name: String,
    family: Family,
}
