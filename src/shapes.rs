pub struct Shape {
    width: u8,
    height: u8,
}

impl Shape {
    pub fn new() -> Self {
        Self {
            width: 20,
            height: 20,
        }
    }
    
    pub fn create_o() -> Vec<Vec<u8>> {
        vec![
            vec![1, 1],
            vec![1, 1],
        ]
    }

    pub fn create_i() -> Vec<Vec<u8>> {
        vec![
            vec![1],
            vec![1],
            vec![1],
            vec![1],
        ]
    }

    pub fn create_s() -> Vec<Vec<u8>> {
        vec![
            vec![0, 1, 1],
            vec![1, 1, 0],
        ]
    }

    pub fn create_z() -> Vec<Vec<u8>> {
        vec![
            vec![1, 1, 0],
            vec![0, 1, 1],
        ]
    }

    pub fn create_t() -> Vec<Vec<u8>> {
        vec![
            vec![1, 1, 1],
            vec![0, 1, 0],
        ]
    }
    
    pub fn create_l() -> Vec<Vec<u8>> {
        vec![
            vec![1, 0],
            vec![1, 0],
            vec![1, 1],
        ]
    }
    
    pub fn create_j() -> Vec<Vec<u8>> {
        vec![
            vec![0, 1],
            vec![0, 1],
            vec![1, 1],
        ]
    }
}
