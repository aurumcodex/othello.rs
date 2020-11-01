// moves.rs

pub struct Move {
    pub cell: usize,
    pub num_flips: isize,
    pub direction: isize,
}

pub trait Movelist {
    fn get_legal_move(&self, index: isize, dir: isize, color: i8) -> Move;

    fn generate_moves(&self, color: i8) -> Vec<Move>;
}

impl Move {
    pub fn check_dir(index: isize, dir: isize) -> bool {
        true
    }

    pub fn check_border(&self) -> bool {
        true
    }

    pub fn get_cells(moveset: Vec<Move>) -> Vec<usize> {
        let mut cells: Vec<usize> = Vec::new();

        cells
    }

    pub fn print(&self, color: i8) {}
}
