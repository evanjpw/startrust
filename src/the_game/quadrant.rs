use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug)]
pub struct Quadrant(u8, u8);

// TODO: Maybe allow invalid quadrants?
impl Quadrant {
    pub(crate) fn new(x: u8, y: u8) -> Self {
        if x > 7 || y > 7 {
            panic!(
                "Could not create quadrant ({}, {}), value out of range",
                x, y
            )
        }
        Self(x, y)
    }

    fn values(&self) -> (u8, u8) {
        (self.0, self.1)
    }

    pub(crate) fn is_in_range(&self) -> bool {
        // Original definition: `(q1<0)||(q1>7)||(q2<0)||(q2>7)`
        // This quadrant can never be out of range, so it's always true
        true
    }

    pub(crate) fn x(&self) -> u8 {
        self.0
    }

    pub(crate) fn y(&self) -> u8 {
        self.1
    }
}

impl Index<Quadrant> for QuadrantMap {
    type Output = i16;

    fn index(&self, index: Quadrant) -> &Self::Output {
        let (q1, q2) = index.values();
        &self.quad[q1 as usize][q2 as usize]
    }
}

impl IndexMut<Quadrant> for QuadrantMap {
    fn index_mut(&mut self, index: Quadrant) -> &mut Self::Output {
        let (q1, q2) = index.values();
        &mut self.quad[q1 as usize][q2 as usize]
    }
}

pub struct QuadrantMap {
    quad: Vec<Vec<i16>>,
}

impl QuadrantMap {
    pub(crate) fn new() -> Self {
        Self {
            quad: vec![vec![0i16; 8]; 8],
        }
    }
}
