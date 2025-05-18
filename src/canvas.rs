use crate::bag::Bag;
use crate::constants::{PLAYGROUND_WIDTH};
use crate::shapes::{Matrix, Shape};
use iced::widget::canvas;
use iced::widget::canvas::{Cache, Geometry};
use iced::{Rectangle, Renderer, Theme, mouse};
use crate::playground::Playground;

#[derive(Debug, Default)]
pub struct State {
    pub now: chrono::DateTime<chrono::Local>,
    pub playground: Cache,
    pub bag: Bag,
    pub game_space: Matrix,
    pub tick_rate_ms: u64,
    pub level: u32,
    pub rows_cleared: u32,
    pub score: u32,
    pub is_running: bool,
    pub next_item: Shape,
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
            
            if !self.is_running {
              playground.render_game_paused(frame);
            }

            playground.render_bricks(frame, &self.game_space);

           playground.render_lines(frame);
        });
        vec![playground]
    }
}
