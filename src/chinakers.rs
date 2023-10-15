use std::fmt::Display;


#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Void,
    Empty,
    Piece,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Piece => 'o',
            Cell::Empty => '.',
            Cell::Void  => ' ',
        })
    }
}


#[derive(Clone, Copy)]
pub enum MoveDirection {
    Up,
    Left,
    Down,
    Right,
}

impl MoveDirection {
    pub fn iter() -> impl Iterator<Item = MoveDirection> {
        [MoveDirection::Up, MoveDirection::Down, MoveDirection::Left, MoveDirection::Right].iter().copied()
    }
}


#[derive(Clone, Copy)]
pub struct Position {
    pub x: i8, pub y: i8,
}


#[derive(Clone, Copy)]
pub struct Move {
    pub position: Position,
    pub direction: MoveDirection,
}

impl Move {
    pub const fn victim_position(&self) -> Position {
        let p = self.position;
        match self.direction {
            MoveDirection::Up => {Position {x: p.x, y: p.y - 1}},
            MoveDirection::Left => {Position {x: p.x - 1, y: p.y}},
            MoveDirection::Down => {Position {x: p.x, y: p.y + 1}},
            MoveDirection::Right => {Position {x: p.x + 1, y: p.y}},
        }
    }

    pub const fn landing_position(&self) -> Position {
        let p = self.position;
        match self.direction {
            MoveDirection::Up => {Position {x: p.x, y: p.y - 2}},
            MoveDirection::Left => {Position {x: p.x - 2, y: p.y}},
            MoveDirection::Down => {Position {x: p.x, y: p.y + 2}},
            MoveDirection::Right => {Position {x: p.x + 2, y: p.y}},
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
pub struct Field {
    data: [Cell; 49],
}


impl Field {
    pub fn new() -> Field {
        let mut data = [Cell::Piece; 49];
        (0_u8..7_u8).for_each(|x| {
            (0_u8..7_u8).for_each(|y| {
                let x_outer = x < 2 || x > 4;
                let y_outer = y < 2 || y > 4;
                if x_outer && y_outer {
                    data[(x + y * 7) as usize] = Cell::Void;
                }
            });
        });
        data[24] = Cell::Empty; // Empty center;
        Field {data}
    }

    pub fn get_cell(&self, position: Position) -> Option<Cell> {
        if !Self::is_valid_position(position) { return None }
        return Some(self.data[(position.x + position.y * 7) as usize]);
    }

    pub fn set_cell(&mut self, position: Position, cell: Cell) -> Result<(), ()> {
        if !Self::is_valid_position(position) { return Err(()) }
        self.data[(position.x + position.y * 7) as usize] = cell;
        Ok(())
    }

    pub const fn is_valid_position(position: Position) -> bool {
        let (x, y) = (position.x, position.y);
        if x < 0 || y < 0 || x > 6 || y > 6 { return false }
        let (x_outer, y_outer) = (x < 2 || x > 4, y < 2 || y > 4);
        !x_outer || !y_outer
    }

    pub fn make_move(&mut self, m: Move) -> Result<(), ()> {
        if !self.is_valid_move(m) { return Err(()) }
        self.set_cell(m.position, Cell::Empty)?;
        self.set_cell(m.victim_position(), Cell::Empty)?;
        self.set_cell(m.landing_position(), Cell::Piece)?;
        Ok(())
    }

    pub fn is_valid_move(&self, m: Move) -> bool {
        let position = m.position;
        let cell_option = self.get_cell(position);
        if let Some(cell) = cell_option {
            if cell != Cell::Piece { return false }
        }
        else { return false }

        if !self.get_cell(m.victim_position()).is_some_and(|x| x == Cell::Piece) { return false }
        if !self.get_cell(m.landing_position()).is_some_and(|x| x == Cell::Empty) { return false }
        true
    }

    pub fn available_moves(&self) -> Vec<Move> {
        let mut moves = Vec::default();

        self.get_pieces().iter().for_each(|&position| {
            MoveDirection::iter().for_each(|direction| {
                let m = Move {position, direction};
                if !self.is_valid_move(m) { return }
                moves.push(m);
            });
        });

        moves
    }

    pub fn eval_heuristic_for_move(&self, m: Move) -> f32 {
        let mut f = self.clone();
        let _ = f.make_move(m);
        return f.eval_heuristic();
    }

    fn get_pieces(&self) -> Vec<Position> {
        let mut pieces = Vec::new();
        (0_i8..7_i8).for_each(|x| {
            (0_i8..7_i8).for_each(|y| {
                let (x_outer, y_outer) = (x < 2 || x > 4, y < 2 || y > 4);
                if x_outer && y_outer { return }
                pieces.push(Position {x, y});
            });
        });
        pieces.retain(|&position| self.get_cell(position).is_some_and(|x| x == Cell::Piece));
        return pieces;
    }
    
    pub fn is_solved(&self) -> bool {
        self.count_pieces() <= 1
    }

    fn count_pieces(&self) -> u8 {
        self.get_pieces().len() as u8
    }

    pub fn eval_heuristic(&self) -> f32 {
        let base = (self.count_pieces() as f32) - 1.0;
        let available_moves = self.available_moves().len() as f32;
        let manhattan_distance_sum = self.manhattan_distance_sum() as f32;
        //base * (1.0 + manhattan_distance_sum * 10.0)
        base + manhattan_distance_sum - available_moves
    }

    pub fn manhattan_distance_sum(&self) -> i32 {
        let mut sum = 0;
        let center_position = Position { x: 3, y: 3 };
        (0_i8..7_i8).for_each(|x| {
            (0_i8..7_i8).for_each(|y| {
                let position = Position { x, y };
                let cell = self.get_cell(position);
                let Some(cell) = cell
                else { return };
                if cell != Cell::Piece { return }
                let manhattan_distance = (center_position.x - position.x).abs() + (center_position.y - position.y).abs();
                sum += manhattan_distance as i32;
            });
        });
        sum
    }

    pub fn display(&self) {
        let data = self.data;
        (0..49).step_by(7).for_each(|i| {
            println!("{} {} {} {} {} {} {}", data[i], data[i + 1], data[i + 2], data[i + 3], data[i + 4], data[i + 5], data[i + 6]);
        });
    }
}

