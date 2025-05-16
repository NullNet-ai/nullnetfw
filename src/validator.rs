use crate::{
    chain::{ChainType, Hook},
    system::{NullnetSystem, Version},
    table::Family,
};

#[derive(Clone, Debug)]
pub struct Validator {
    system: NullnetSystem,
}

impl Validator {
    pub fn new(system: NullnetSystem) -> Self {
        Self { system }
    }

    /// Determines if a given `chain_type` is valid for the specified `family`.
    ///
    /// This method checks whether the specified chain type is supported within the given family.
    ///
    /// # References
    /// - https://wiki.nftables.org/wiki-nftables/index.php/Quick_reference-nftables_in_10_minutes#Chains
    ///
    /// # Arguments
    /// - `chain_type`: The type of chain.
    /// - `family`: The family for which the chain is being defined.
    ///
    /// # Returns
    /// - `true` if the combination is supported, otherwise `false`.
    pub fn is_chain_type_allowed(&self, chain_type: ChainType, family: Family) -> bool {
        use ChainType::*;
        use Family::*;

        match chain_type {
            Filter => matches!(family, Ip | Ip6 | Inet | Arp | Bridge),
            Route => matches!(family, Ip | Ip6),
            Nat => matches!(family, Ip | Ip6),
        }
    }

    /// Determines if a given `hook` is allowed for the specified `chain_type` and `family`.
    ///
    /// This method checks whether a hook can be used for a given combination of chain type and family,
    /// also considering kernel and nftables version constraints for certain hooks.
    ///
    /// # References
    /// - https://wiki.nftables.org/wiki-nftables/index.php/Netfilter_hooks#Hooks_by_family_and_chain_type
    ///
    /// # Arguments
    /// - `hook`: The netfilter hook.
    /// - `chain_type`: The chain type to be used with the hook.
    /// - `family`: The network family.
    ///
    /// # Returns
    /// - `true` if the hook is valid in the given context, otherwise `false`.
    pub fn is_hook_allowed(&self, hook: Hook, chain_type: ChainType, family: Family) -> bool {
        use ChainType::*;
        use Family::*;
        use Hook::*;

        match family {
            Inet => match chain_type {
                Filter => match hook {
                    Ingress => {
                        self.system.nftables_ver >= Version::new(0, 9, 7)
                            && self.system.linux_ver >= Version::new(5, 10, 0)
                    }
                    Egress => false,
                    _ => true,
                },
                Nat => matches!(hook, Prerouting | Input | Output | Postrouting),
                Route => matches!(hook, Output),
            },
            Ip6 | Ip => match chain_type {
                Filter => {
                    matches!(hook, Prerouting | Forward | Input | Output | Postrouting)
                }
                Nat => matches!(hook, Prerouting | Input | Output | Postrouting),
                Route => matches!(hook, Output),
            },

            Arp => match chain_type {
                Filter => matches!(hook, Input | Output),
                _ => false,
            },
            Bridge => match chain_type {
                Filter => {
                    matches!(hook, Input | Output | Prerouting | Postrouting | Forward)
                }
                _ => false,
            },
            Netdev => match chain_type {
                Filter => match hook {
                    Ingress => {
                        self.system.nftables_ver >= Version::new(0, 6, 0)
                            && self.system.linux_ver >= Version::new(4, 2, 0)
                    }
                    Egress => {
                        self.system.nftables_ver >= Version::new(1, 0, 1)
                            && self.system.linux_ver >= Version::new(5, 16, 0)
                    }
                    _ => false,
                },
                _ => false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::Version;

    fn create_validator(linux: (u32, u32, u32), nftables: (u32, u32, u32)) -> Validator {
        let system = NullnetSystem {
            linux_ver: Version::new(linux.0, linux.1, linux.2),
            nftables_ver: Version::new(nftables.0, nftables.1, nftables.2),
        };
        Validator::new(system)
    }

    #[test]
    fn test_chain_type_allowed() {
        let validator = create_validator((0, 0, 0), (0, 0, 0));

        assert!(validator.is_chain_type_allowed(ChainType::Filter, Family::Ip));
        assert!(validator.is_chain_type_allowed(ChainType::Filter, Family::Ip6));
        assert!(validator.is_chain_type_allowed(ChainType::Filter, Family::Inet));
        assert!(validator.is_chain_type_allowed(ChainType::Filter, Family::Arp));
        assert!(validator.is_chain_type_allowed(ChainType::Filter, Family::Bridge));
        assert!(!validator.is_chain_type_allowed(ChainType::Filter, Family::Netdev));

        assert!(validator.is_chain_type_allowed(ChainType::Nat, Family::Ip));
        assert!(validator.is_chain_type_allowed(ChainType::Nat, Family::Ip6));
        assert!(!validator.is_chain_type_allowed(ChainType::Nat, Family::Inet));
        assert!(!validator.is_chain_type_allowed(ChainType::Nat, Family::Arp));
        assert!(!validator.is_chain_type_allowed(ChainType::Nat, Family::Bridge));
        assert!(!validator.is_chain_type_allowed(ChainType::Nat, Family::Netdev));

        assert!(validator.is_chain_type_allowed(ChainType::Route, Family::Ip));
        assert!(validator.is_chain_type_allowed(ChainType::Route, Family::Ip6));
        assert!(!validator.is_chain_type_allowed(ChainType::Route, Family::Inet));
        assert!(!validator.is_chain_type_allowed(ChainType::Route, Family::Arp));
        assert!(!validator.is_chain_type_allowed(ChainType::Route, Family::Bridge));
        assert!(!validator.is_chain_type_allowed(ChainType::Route, Family::Netdev));
    }

    #[test]
    fn test_ip_hooks() {
        let validator = create_validator((0, 0, 0), (0, 0, 0));

        assert!(validator.is_hook_allowed(Hook::Prerouting, ChainType::Filter, Family::Ip));
        assert!(validator.is_hook_allowed(Hook::Input, ChainType::Filter, Family::Ip));
        assert!(validator.is_hook_allowed(Hook::Forward, ChainType::Filter, Family::Ip));
        assert!(validator.is_hook_allowed(Hook::Output, ChainType::Filter, Family::Ip));
        assert!(validator.is_hook_allowed(Hook::Postrouting, ChainType::Filter, Family::Ip));
        assert!(!validator.is_hook_allowed(Hook::Ingress, ChainType::Filter, Family::Ip));
        assert!(!validator.is_hook_allowed(Hook::Egress, ChainType::Filter, Family::Ip));

        assert!(validator.is_hook_allowed(Hook::Prerouting, ChainType::Filter, Family::Ip6));

        assert!(validator.is_hook_allowed(Hook::Prerouting, ChainType::Nat, Family::Ip));
        assert!(validator.is_hook_allowed(Hook::Input, ChainType::Nat, Family::Ip));
        assert!(!validator.is_hook_allowed(Hook::Forward, ChainType::Nat, Family::Ip));
        assert!(validator.is_hook_allowed(Hook::Output, ChainType::Nat, Family::Ip));
        assert!(validator.is_hook_allowed(Hook::Postrouting, ChainType::Nat, Family::Ip));

        assert!(!validator.is_hook_allowed(Hook::Prerouting, ChainType::Route, Family::Ip));
        assert!(!validator.is_hook_allowed(Hook::Input, ChainType::Route, Family::Ip));
        assert!(!validator.is_hook_allowed(Hook::Forward, ChainType::Route, Family::Ip));
        assert!(validator.is_hook_allowed(Hook::Output, ChainType::Route, Family::Ip));
        assert!(!validator.is_hook_allowed(Hook::Postrouting, ChainType::Route, Family::Ip));
    }

    #[test]
    fn test_inet_hooks() {
        let validator_old = create_validator((1, 9, 99), (0, 1, 6));
        assert!(!validator_old.is_hook_allowed(Hook::Ingress, ChainType::Filter, Family::Inet));

        let validator_min = create_validator((5, 10, 0), (0, 9, 7));
        assert!(validator_min.is_hook_allowed(Hook::Ingress, ChainType::Filter, Family::Inet));

        let validator_new = create_validator((6, 0, 0), (1, 0, 0));
        assert!(validator_new.is_hook_allowed(Hook::Ingress, ChainType::Filter, Family::Inet));

        assert!(!validator_new.is_hook_allowed(Hook::Egress, ChainType::Filter, Family::Inet));

        assert!(validator_old.is_hook_allowed(Hook::Prerouting, ChainType::Filter, Family::Inet));
        assert!(validator_old.is_hook_allowed(Hook::Input, ChainType::Filter, Family::Inet));

        assert!(validator_old.is_hook_allowed(Hook::Prerouting, ChainType::Nat, Family::Inet));
        assert!(!validator_old.is_hook_allowed(Hook::Forward, ChainType::Nat, Family::Inet));
    }

    #[test]
    fn test_arp_hooks() {
        let validator = create_validator((0, 0, 0), (0, 0, 0));

        assert!(validator.is_hook_allowed(Hook::Input, ChainType::Filter, Family::Arp));
        assert!(validator.is_hook_allowed(Hook::Output, ChainType::Filter, Family::Arp));

        assert!(!validator.is_hook_allowed(Hook::Prerouting, ChainType::Filter, Family::Arp));
        assert!(!validator.is_hook_allowed(Hook::Input, ChainType::Nat, Family::Arp));
        assert!(!validator.is_hook_allowed(Hook::Output, ChainType::Route, Family::Arp));
    }

    #[test]
    fn test_bridge_hooks() {
        let validator = create_validator((0, 0, 0), (0, 0, 0));

        assert!(validator.is_hook_allowed(Hook::Prerouting, ChainType::Filter, Family::Bridge));
        assert!(validator.is_hook_allowed(Hook::Input, ChainType::Filter, Family::Bridge));
        assert!(validator.is_hook_allowed(Hook::Forward, ChainType::Filter, Family::Bridge));
        assert!(validator.is_hook_allowed(Hook::Output, ChainType::Filter, Family::Bridge));
        assert!(validator.is_hook_allowed(Hook::Postrouting, ChainType::Filter, Family::Bridge));

        assert!(!validator.is_hook_allowed(Hook::Prerouting, ChainType::Nat, Family::Bridge));
        assert!(!validator.is_hook_allowed(Hook::Input, ChainType::Route, Family::Bridge));
    }

    #[test]
    fn test_netdev_hooks() {
        let validator_old = create_validator((4, 1, 0), (0, 5, 0));
        assert!(!validator_old.is_hook_allowed(Hook::Ingress, ChainType::Filter, Family::Netdev));
        assert!(!validator_old.is_hook_allowed(Hook::Egress, ChainType::Filter, Family::Netdev));

        let validator_ingress = create_validator((4, 2, 0), (0, 6, 0));
        assert!(validator_ingress.is_hook_allowed(
            Hook::Ingress,
            ChainType::Filter,
            Family::Netdev
        ));
        assert!(!validator_ingress.is_hook_allowed(
            Hook::Egress,
            ChainType::Filter,
            Family::Netdev
        ));

        let validator_full = create_validator((5, 16, 0), (1, 0, 1));
        assert!(validator_full.is_hook_allowed(Hook::Ingress, ChainType::Filter, Family::Netdev));
        assert!(validator_full.is_hook_allowed(Hook::Egress, ChainType::Filter, Family::Netdev));

        assert!(!validator_full.is_hook_allowed(Hook::Ingress, ChainType::Nat, Family::Netdev));
        assert!(!validator_full.is_hook_allowed(Hook::Egress, ChainType::Route, Family::Netdev));
    }

    #[test]
    fn test_edge_cases() {
        let validator = create_validator((0, 0, 0), (0, 0, 0));

        assert!(!validator.is_hook_allowed(Hook::Input, ChainType::Route, Family::Bridge));
        assert!(!validator.is_hook_allowed(Hook::Forward, ChainType::Nat, Family::Ip));

        let validator_exact = create_validator((5, 10, 0), (0, 9, 7));
        assert!(validator_exact.is_hook_allowed(Hook::Ingress, ChainType::Filter, Family::Inet));
    }
}
