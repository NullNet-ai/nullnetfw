// @TODO: Double check if those are all the values (i doubt)

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Priority {
    NfIpPriConntrackDefrag = -400,
    NfIpPriRaw = -300,
    NfIpPriSelinuxFirst = -225,
    NfIpPriConntrack = -200,
    NfIpPriMangle = -150,
    NfIpPriNatDst = -100,
    NfIpPriFilter = 0,
    NfIpPriSecurity = 50,
    NfIpPriNatSrc = 100,
    NfIpPriSelinuxLast = 225,
    NfIpPriConntrackHelper = 300,
}

impl TryFrom<i32> for Priority {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use Priority::*;
        match value {
            -400 => Ok(NfIpPriConntrackDefrag),
            -300 => Ok(NfIpPriRaw),
            -225 => Ok(NfIpPriSelinuxFirst),
            -200 => Ok(NfIpPriConntrack),
            -150 => Ok(NfIpPriMangle),
            -100 => Ok(NfIpPriNatDst),
            0 => Ok(NfIpPriFilter),
            50 => Ok(NfIpPriSecurity),
            100 => Ok(NfIpPriNatSrc),
            225 => Ok(NfIpPriSelinuxLast),
            300 => Ok(NfIpPriConntrackHelper),
            _ => Err(()),
        }
    }
}
