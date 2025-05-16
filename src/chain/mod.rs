pub use chain_policy::ChainPolicy;
pub use chain_type::ChainType;
pub use hook::Hook;
pub use priority::Priority;

mod chain_policy;
mod chain_type;
mod hook;
mod priority;

pub struct Chain {
    r#type: ChainType,
    priority: Option<Priority>,
    policy: ChainPolicy,
}
