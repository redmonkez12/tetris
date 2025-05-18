use rand::seq::SliceRandom;
use crate::shapes::{Shape};
use crate::types::Shapes;

#[derive(Default)]
#[derive(Debug)]
pub struct Bag {
    pub items: Shapes,
}

impl Bag {
    pub fn new() -> Self {
        let default_items = Bag::create_items();
        
        Self {
            items: default_items,
        }
    }
    
    fn create_items() -> Shapes {
        let mut default_items = vec![
            Shape::create_o(),
            Shape::create_i(),
            Shape::create_s(),
            Shape::create_z(),
            Shape::create_t(),
            Shape::create_l(),
            Shape::create_j(),
        ];
        default_items.shuffle(&mut rand::rng());
        
        default_items
    }

    pub fn refill(&mut self) {
        self.items = Bag::create_items();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_bag() {
        let bag = Bag::new();
        assert_eq!(bag.default_items.len(), 7);
        assert_eq!(bag.items.len(), 7);
        assert_eq!(bag.items, bag.default_items);
    }
    
    #[test]
    fn test_refill() {
        
    }
}