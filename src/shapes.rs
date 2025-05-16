use iced::Color;

#[derive(Clone, Copy, Debug)]
pub struct Brick {
    pub color: Color,
    pub moving: bool,
}

impl Brick {
    pub fn new(color: Color, moving: bool) -> Self {
        Brick { color, moving}
    }
}

pub type Matrix = Vec<Vec<Option<Brick>>>;

#[derive(Clone, Default, Debug)]
pub struct Shape {
    pub matrix: Matrix,
}

impl Shape {
    pub fn new(matrix: Vec<Vec<Option<Brick>>>) -> Self {
        Self { matrix }
    }

    pub fn create_o() -> Self {
        let color = Color::from_rgb(1.0, 1.0, 0.0);
        let b = Some(Brick::new(color, false));
        Self::new(vec![vec![b, b], vec![b, b]])
    }

    pub fn create_i() -> Self {
        let color = Color::from_rgb(0.0, 1.0, 1.0);
        let b = Some(Brick::new(color, false));
        let e = None;

        Self::new(
            vec![
                vec![e, e, e, e],
                vec![b, b, b, b],
                vec![e, e, e, e],
                vec![e, e, e, e],
            ],
        )
    }

    pub fn create_s() -> Self {
        let color = Color::from_rgb(0.0, 1.0, 0.0);
        let b = Some(Brick::new(color, false));
        let e = None;

        Self::new(vec![vec![e, e, e], vec![e, b, b], vec![b, b, e]])
    }

    pub fn create_z() -> Self {
        let color = Color::from_rgb(1.0, 0.0, 0.0);
        let b = Some(Brick::new(color, false));
        let e = None;

        Self::new(vec![vec![e, e, e], vec![b, b, e], vec![e, b, b]])
    }

    pub fn create_t() -> Self {
        let color = Color::from_rgb(0.5, 0.0, 0.5);
        let b = Some(Brick::new(color, false));
        let e = None;

        Self::new(vec![vec![e, e, e], vec![b, b, b], vec![e, b, e]])
    }

    pub fn create_l() -> Self {
        let color = Color::from_rgb(1.0, 0.5, 0.0);
        let b = Some(Brick::new(color, false));
        let e = None;

        Self::new(
            vec![
                vec![e, e, e, e],
                vec![e, b, e, e],
                vec![e, b, e, e],
                vec![e, b, b, e],
            ],
        )
    }

    pub fn create_j() -> Self {
        let color = Color::from_rgb(0.0, 0.0, 1.0);
        let b = Some(Brick::new(color, false));
        let e = None;

        Self::new(
            vec![
                vec![e, e, e, e],
                vec![e, e, b, e],
                vec![e, e, b, e],
                vec![e, b, b, e],
            ],
        )
    }
}
