mod version;
mod vparser;

use std::io;
use version::Version;
use vparser::VParser;

#[derive(Debug, Clone, Copy)]
pub struct NullnetSystem {
    lver: Version,
    nver: Version,
}

impl NullnetSystem {
    pub fn new() -> io::Result<Self> {
        let lver = VParser::parse_linux_version()?;
        let nver = VParser::parse_nftables_version()?;

        Ok(Self { lver, nver })
    }

    pub fn get_linux_version(&self) -> Version {
        self.lver
    }

    pub fn get_nftables_version(&self) -> Version {
        self.nver
    }

    pub fn greeting(&self) {
        println!("{GREETING}");
        println!("{:<20}{}", "Linux version", self.get_linux_version());
        println!("{:<20}{}", "NFTables version", self.get_nftables_version());
    }
}

const GREETING: &str = r#"
  _   _ _    _ _      _      _   _ ______ _______ 
 | \ | | |  | | |    | |    | \ | |  ____|__   __|
 |  \| | |  | | |    | |    |  \| | |__     | |   
 | . ` | |  | | |    | |    | . ` |  __|    | |   
 | |\  | |__| | |____| |____| |\  | |____   | |   
 |_| \_|\____/|______|______|_| \_|______|  |_|   
"#;
