#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn moved(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position { x: self.x - 1, y: self.y },
            Direction::Down => Position { x: self.x + 1, y: self.y },
            Direction::Left => Position { x: self.x, y: self.y - 1 },
            Direction::Right => Position { x: self.x, y: self.y + 1 },
        }
    }

    pub fn subtract(&self, pos: Position) -> Position {
        Position { x: self.x - pos.x, y: self.y - pos.y }
    }

    pub fn add(&self, pos: Position) -> Position {
        Position { x: self.x + pos.x, y: self.y + pos.y }
    }

    pub fn multiply(&self, multiplier: i32) -> Position {
        Position { x: self.x * multiplier, y: self.y * multiplier }
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction { Left, Right, Up, Down }

impl Direction {
    pub fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];
