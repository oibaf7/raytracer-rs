use crate::vector::Color;
use std::cmp::Ordering;
use std::f64::consts::PI;

pub struct Tile {
    id: usize,
    y: usize,
}

impl Tile {
    pub fn new(id: usize, y: usize) -> Self {
        Self { id, y }
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

pub struct ResultTile {
    id: usize,
    colors: Vec<Color>,
}

impl Eq for ResultTile {}

impl PartialEq<Self> for ResultTile {
    fn eq(&self, other: &Self) -> bool {
        other.id == self.id
    }
}

impl PartialOrd<Self> for ResultTile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ResultTile {
    fn cmp(&self, other: &Self) -> Ordering {
        if other.id < self.id {
            Ordering::Greater
        } else if other.id == self.id {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl ResultTile {
    pub fn new(id: usize, colors: Vec<Color>) -> Self {
        Self { id, colors }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn colors(&self) -> &Vec<Color> {
        &self.colors
    }
}
