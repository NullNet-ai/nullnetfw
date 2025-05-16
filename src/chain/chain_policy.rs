use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ChainPolicy {
    #[default]
    Accept,
    Drop,
}

impl FromStr for ChainPolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "accept" => Ok(ChainPolicy::Accept),
            "drop" => Ok(ChainPolicy::Drop),
            other => Err(format!("Unknown chain policy: {}", other)),
        }
    }
}

impl fmt::Display for ChainPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ChainPolicy::Accept => "accept",
            ChainPolicy::Drop => "drop",
        };

        write!(f, "{}", s)
    }
}
