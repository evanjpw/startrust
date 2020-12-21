use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Default)]
pub struct StarDate(u16);

impl StarDate {
    pub fn new(t: u16) -> Self {
        Self(t)
    }
}

impl Display for StarDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<I: Into<u16>> ops::Add<I> for StarDate {
    type Output = StarDate;

    fn add(self, rhs: I) -> StarDate {
        StarDate(self.0 + rhs.into())
    }
}

impl<I: Into<u16>> ops::AddAssign<I> for StarDate {
    fn add_assign(&mut self, rhs: I) {
        *self = StarDate(self.0 + rhs.into());
    }
}

impl ops::Sub<StarDate> for StarDate {
    type Output = u16;

    fn sub(self, rhs: StarDate) -> u16 {
        self.0 - rhs.0
    }
}
