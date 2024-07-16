use rand::{thread_rng, Rng};

#[derive(Clone, PartialEq, Debug)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        matches!(self, Cell::Alive)
    }

    pub fn is_dead(&self) -> bool {
        !self.is_alive()
    }

    pub fn swap_state(&mut self) {
        if self.is_alive() {
            *self = Cell::Dead;
        } else {
            *self = Cell::Alive;
        }
    }

    pub fn create(is_dead: bool) -> Cell {
        if is_dead {
            Cell::Dead
        } else {
            Cell::Alive
        }
    }
}

pub type CellMatrix = Vec<Vec<Cell>>;
pub type Seeder = Box<dyn Fn(u32, u32) -> bool>;

pub fn apply_cell_rules(neighbors_count: i32, is_dead: bool) -> bool {
    #![allow(clippy::match_like_matches_macro)]
    match (neighbors_count, is_dead) {
        (3, true) => false,
        (2 | 3, false) => false,
        _ => true,
    }
}

pub fn get_random_seeder(percentage: u32) -> Seeder {
    Box::new(move |_, _| thread_rng().gen_range(0..101) <= percentage)
}
