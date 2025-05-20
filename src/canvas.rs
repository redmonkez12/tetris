use crate::bag::Bag;
use crate::playground::Playground;
use crate::tetromino::Tetromino;
use crate::types::{Matrix, TimeLocal};
use iced::widget::canvas;
use iced::widget::canvas::{Cache, Geometry};
use iced::{Rectangle, Renderer, Theme, mouse};

#[derive(Debug, Default)]
pub struct State {
    pub now: TimeLocal,
    pub playground: Cache,
    pub bag: Bag,
    pub game_space: Matrix,
    pub tick_rate_ms: u64,
    pub level: u32,
    pub rows_cleared: u32,
    pub score: u32,
    pub is_running: bool,
    pub game_over: bool,
    pub next_item: Tetromino,
}

impl<Message> canvas::Program<Message> for State {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let playground = self.playground.draw(renderer, bounds.size(), |frame| {
            let half = bounds.width / 2.0;

            let mut playground = Playground::new(half);

            playground.render_level(frame, self.level);
            playground.render_score(frame, self.score);
            playground.render_next_brick(frame, &self.next_item);

            playground.render_bricks(frame, &self.game_space);

            if !self.is_running {
                playground.render_game_paused(frame);
            }

            if self.game_over {
                playground.render_game_over(frame);
            }

            playground.render_lines(frame);
        });
        vec![playground]
    }
}
