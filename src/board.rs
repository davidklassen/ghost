use std::fmt;

#[derive(Clone)]
pub enum Stone {
    White,
    Black,
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Stone::White => 'o',
            Stone::Black => 'x',
        };
        write!(f, "{}", printable)
    }
}

#[derive(Clone)]
pub enum SlotState {
    Stone(Stone),
    Empty,
}

impl fmt::Display for SlotState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            SlotState::Stone(ref stone) => format!("{}", stone),
            SlotState::Empty => "┼".to_string(),
        };
        write!(f, "{}", printable)
    }
}

pub struct Coords {
    pub x: usize,
    pub y: usize,
}

pub struct Board {
    position: Vec<Vec<SlotState>>,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board{
            position: vec![vec![SlotState::Empty; size]; size]
        }
    }

    pub fn set_slot(&mut self, state: SlotState, coords: Coords) {
        self.position[coords.x][coords.y] = state
    }

    fn render(&self) -> String {
        let size = self.position.len();
        let mut printable = String::new();
        for i in 0..(size * size + size - 1) {
            let x = i % (size + 1);
            let y = i / (size + 1);
            if x % (size + 1) == size {
                printable.push_str("\n");
            } else if x == 0 && y == 0 {
                printable.push_str("┌");
            } else if x == size - 1 && y == 0 {
                printable.push_str("┐");
            } else if x == size - 1 && y == size - 1 {
                printable.push_str("┘");
            } else if x == 0 && y == size - 1 {
                printable.push_str("└");
            } else if x == 0 {
                printable.push_str("├");
            } else if y == 0 {
                printable.push_str("┬");
            } else if x == size - 1 {
                printable.push_str("┤");
            } else if y == size - 1 {
                printable.push_str("┴");
            } else {
                printable.push_str(&format!("{}", self.position[x][y]));
            }
        }
        return printable
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}
