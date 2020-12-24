use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Default)]
pub struct StarDate(i32);

impl StarDate {
    pub fn new(t: i32) -> Self {
        Self(t)
    }
}

impl Display for StarDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<I: Into<i32>> ops::Add<I> for StarDate {
    //u16
    type Output = StarDate;

    fn add(self, rhs: I) -> StarDate {
        StarDate(self.0 + rhs.into())
    }
}

impl<I: Into<i32>> ops::AddAssign<I> for StarDate {
    //u16
    fn add_assign(&mut self, rhs: I) {
        *self = StarDate(self.0 + rhs.into());
    }
}

impl ops::Sub<StarDate> for StarDate {
    type Output = i32; //u16

    fn sub(self, rhs: StarDate) -> i32 {
        self.0 - rhs.0
    } //u16
}
