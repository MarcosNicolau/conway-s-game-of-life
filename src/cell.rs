#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
    pub pos: Pos,
    pub is_dead: bool,
}

pub type CellMatrix = Vec<Vec<Cell>>;
pub type Seeder = fn(row_idx: u32, col_idx: u32) -> bool;

pub fn apply_cell_rules(neighbors_count: i32, is_dead: bool) -> bool {
    match (neighbors_count, is_dead) {
        (3, true) => false,
        (2 | 3, false) => false,
        _ => true,
    }
}
