mod version;
mod vparser;

use std::io;
pub use version::Version;
use vparser::VParser;

#[derive(Debug, Clone, Copy)]
pub struct NullnetSystem {
    // Linus version
    pub linux_ver: Version,
    // NFTables version
    pub nftables_ver: Version,
}

impl NullnetSystem {
    pub fn new() -> io::Result<Self> {
        let linux_ver = VParser::parse_linux_version()?;
        let nftables_ver = VParser::parse_nftables_version()?;

        Ok(Self {
            linux_ver,
            nftables_ver,
        })
    }

    pub fn greeting(&self) {
        println!("{GREETING}");
        println!("{:<20}{}", "Linux version", self.linux_ver);
        println!("{:<20}{}", "NFTables version", self.nftables_ver);
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
