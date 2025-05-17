use rand::seq::SliceRandom;
use crate::shapes::{Brick, Shape};

#[derive(Default)]
#[derive(Debug)]
pub struct Bag {
    pub items: Vec<Shape>,
    pub default_items: Vec<Shape>,
}

impl Bag {
    pub fn new() -> Self {
        Self {
            default_items: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn refill(&mut self) {
        let mut rng = rand::rng();
        self.items = self.default_items.clone();
        self.items.shuffle(&mut rng);
    }

    pub fn get_item(&mut self) -> Shape {
        let shape = self.items.pop().unwrap();

        if self.items.len() < 1 {
            self.refill();
        }
        
        shape
    }

    pub fn show_next(&self) -> Option<Shape> {
        self.items.last().cloned()
    }
}
