use rand::seq::SliceRandom;
use crate::shapes::{Shape};
use crate::types::Shapes;

#[derive(Default)]
#[derive(Debug)]
pub struct Bag {
    pub items: Shapes,
    pub default_items: Shapes,
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

    pub fn peek(&self) -> Option<Shape> {
        self.items.last().cloned()
    }
 
    pub fn reset(&mut self) {
        self.default_items = vec![
            Shape::create_o(),
            Shape::create_i(),
            Shape::create_s(),
            Shape::create_z(),
            Shape::create_t(),
            Shape::create_l(),
            Shape::create_j(),
        ];
        self.refill();
    }
}
