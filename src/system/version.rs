#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.major
            .cmp(&other.major)
            .then_with(|| self.minor.cmp(&other.minor))
            .then_with(|| self.patch.cmp(&other.patch))
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_without_suffix() {
        let v = Version::new(3, 4, 5);
        assert_eq!(v.to_string(), "3.4.5");
    }

    #[test]
    fn test_version_equality() {
        let v1 = Version::new(1, 2, 3);
        let v2 = Version::new(1, 2, 3);

        assert_eq!(v1, v2);
    }

    #[test]
    fn test_version_ordering() {
        let v1 = Version::new(1, 2, 3);
        let v2 = Version::new(1, 2, 4);
        let v3 = Version::new(1, 3, 0);
        let v4 = Version::new(2, 0, 0);
        let v5 = Version::new(1, 2, 3);

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
        assert_eq!(v1.cmp(&v5), std::cmp::Ordering::Equal);
    }
}
