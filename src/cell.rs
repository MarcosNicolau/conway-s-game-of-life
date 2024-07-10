use rand::{thread_rng, Rng};

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
pub type Seeder = Box<dyn Fn(u32, u32) -> bool>;

pub fn apply_cell_rules(neighbors_count: i32, is_dead: bool) -> bool {
    !matches!((neighbors_count, is_dead), (3, true) | (2 | 3, false))
}

pub fn get_random_seeder(percentage: u32) -> Seeder {
    Box::new(move |_, _| thread_rng().gen_range(0..101) <= percentage)
}
