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

    pub fn get_item(&mut self) -> (Shape, Shape) {
        let shape = self.items.pop().unwrap();

        if self.items.len() < 1 {
            self.refill();
        }

        let next_item = self.items.last().unwrap().clone();
        
        (shape, next_item)
    }

    pub fn refill(&mut self) {
        self.items = Bag::create_items();
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
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_shuffle_creates_different_bags() {
        let bag1 = Bag::new();
        let bag2 = Bag::new();
        assert_ne!(format!("{:?}", bag1.items), format!("{:?}", bag2.items));
    }

    #[test]
    fn test_refill() {
        let mut bag = Bag::new();
        bag.get_item();
        bag.get_item();
        assert_eq!(bag.items.len(), 5);

        bag.refill();
        assert_eq!(bag.items.len(), 7);
    }

    #[test]
    fn test_default_bag() {
        let bag = Bag::new();
        assert_eq!(bag.items.len(), 7);
    }

    #[test]
    fn test_all_shapes_present() {
        let bag = Bag::new();

        let mut colors = HashSet::new();

        for shape in &bag.items {
            for row in &shape.matrix {
                for cell in row {
                    if let Some(brick) = cell {
                        colors.insert(format!("{:?}", brick.color));
                        break;
                    }
                }
            }
        }

        assert_eq!(colors.len(), 7);
    }

    #[test]
    fn test_get_item() {
        let mut bag = Bag::new();
        let original_length = bag.items.len();
        assert_eq!(original_length, 7, "A new bag should have 7 items");

        let (_, next_shape) = bag.get_item();
        assert_eq!(bag.items.len(), 6, "Bag should have 6 items after getting one");
        assert_eq!(next_shape, bag.items.last().unwrap().clone(), "Next shape should match the last in the bag");

        let last_item_before_refill = bag.items[0].clone();

        for i in 0..5 {
            let (_, _) = bag.get_item();
            assert_eq!(bag.items.len(), 5-i, "Bag should decrease by one each time");
        }

        let (_, next_shape_after_refill) = bag.get_item();

        assert_eq!(bag.items.len(), 7, "Bag should refill to 7 items");
        assert_eq!(next_shape_after_refill, bag.items.last().unwrap().clone(),
                   "Next shape after refill should match the last in the new bag");
    }
}