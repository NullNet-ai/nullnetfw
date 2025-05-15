pub use chain_type::ChainType;
pub use hook::Hook;

mod chain_type;
mod hook;

pub struct Chain {
    r#type: ChainType,
}
