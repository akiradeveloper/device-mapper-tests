use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, Ord)]
pub struct Sector(pub u64);
impl Sector {
    pub fn KB(n: u64) -> Sector {
        Sector(2 * n)
    }
    pub fn MB(n: u64) -> Sector {
        Sector((2<<10) * n)
    }
    pub fn GB(n: u64) -> Sector {
        Sector((2<<20) * n)
    }
    pub fn bytes(&self) -> u64 {
        self.0 << 9
    }
    pub fn sectors(&self) -> u64 {
        self.0
    }
}
impl std::ops::Add for Sector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Sector(self.0 + other.0)
    }
}
impl std::ops::Sub for Sector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Sector(self.0 - other.0)
    }
}
impl std::ops::Mul<u64> for Sector {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self {
        Sector(self.0 * rhs)
    }
}
impl PartialOrd for Sector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}
impl PartialEq for Sector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}