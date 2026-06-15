use serde::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GridTile(pub u8);

impl GridTile {
    pub const fn try_next<const GRID_SIZE: usize>(self) -> Option<Self> {
        let n = self.0 + 1;
        if (n as usize) < GRID_SIZE {
            Some(Self(n))
        } else {
            None
        }
    }

    pub const fn try_from_usize<const GRID_SIZE: usize>(n: usize) -> Option<Self> {
        if n < GRID_SIZE {
            Some(Self(n as u8))
        } else {
            None
        }
    }

    pub const fn is_legal<const GRID_SIZE: usize>(self) -> bool {
        (self.0 as usize) < GRID_SIZE
    }

    pub const fn inner_u32(self) -> u32 {
        self.0 as u32
    }

    pub const fn inner_usize(self) -> usize {
        self.0 as usize
    }
}
