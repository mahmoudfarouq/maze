use rltk::{Rltk, GameState, Console};
use rand::{Rng};

use crate::tile::*;
use crate::state::*;
use crate::helpers::*;
use crate::drawer::*;

pub struct RandomizedPrim {
    states: Vec<TileState>,
    disjoint_set: Vec<i32>,
    tree: Vec<i32>,
    stack: Vec<u32>,
    drawer: Drawer,
}
impl RandomizedPrim {
    pub fn new(full_length: usize, start_idx: u32, drawer: Drawer) -> Self {
        let mut stack: Vec<u32>  = Vec::new();
        stack.push(start_idx);
        RandomizedPrim {
            states: vec![TileState::UnDiscovered; full_length],
            disjoint_set: vec![-1; full_length],
            tree: vec![-1; full_length],
            stack,
            drawer,
        }
    }
}
impl State for RandomizedPrim {
    fn update(&mut self) {
        if self.stack.is_empty() { return }

        let head = self.stack.remove(rand::thread_rng().gen_range(0, self.stack.len())) as usize;

        self.disjoint_set[head] = self.tree[head];

        self.states[head] = TileState::Visited;

        for neighbour in neighbours(head as i32) {

            if self.states[neighbour] == TileState::UnDiscovered {
                self.states[neighbour] = TileState::Discovered;
                self.stack.push(neighbour as u32);

                self.tree[neighbour] = head as i32;

            }

        }


    }
}
impl GameState for RandomizedPrim {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        self.update();

        self.drawer.draw(&self.disjoint_set, ctx);
    }
}