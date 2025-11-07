pub enum HexDirection {
    N,
    NE,
    NW,
    S,
    SE,
    SW,
}

impl HexDirection {
    pub fn to_offset(self) -> (i32, i32) {
        use HexDirection::*;
        match self {
            N => (0, 1),
            NE => (-1, 1),
            NW => (-1, 0),
            S => (0, -1),
            SE => (1, -1),
            SW => (1, 0),
        }
    }

    pub fn from_offset(dq: i32, dr: i32) -> Option<Self> {
        use HexDirection::*;
        match (dq, dr) {
            (0, 1) => Some(N),
            (-1, 1) => Some(NE),
            (-1, 0) => Some(NW),
            (0, -1) => Some(S),
            (1, -1) => Some(SE),
            (1, 0) => Some(SW),
            _ => None,
        }
    }

    pub fn all() -> [HexDirection; 6] {
        use HexDirection::*;
        [N, NE, NW, S, SE, SW]
    }
}

pub struct HexRelation {}
