use rand::seq::SliceRandom;

#[derive(Default)]
pub struct Bag {
    pub items: Vec<Vec<Vec<u8>>>,
    pub default_items: Vec<Vec<Vec<u8>>>,
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

    pub fn get_item(&mut self) -> Vec<Vec<u8>> {
        if self.items.len() < 1 {
            self.refill();
        }
    
        self.items.pop().unwrap()
    }
}
