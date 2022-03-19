//! Wtf is this game even
use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;

#[derive(Debug)]
struct QWOPtopus {
    input: InputState,
    player: TheVillian,
}

#[derive(Debug)]
struct TheVillian {
    center: Vec2,
    top_left: Vec2,
    bottom_left: Vec2,
    top_right: Vec2,
    bottom_right: Vec2,
}

impl Default for TheVillian {
    fn default() -> Self {
        let center = Vec2::new(400.0, 400.0);
        let leg_delta = Vec2::new(50.0, 50.0) * 0.3;
        let leg2_delta = Vec2::new(50.0, -50.0) * 0.3;
        Self {
            center,
            top_left: center - leg_delta,
            bottom_left: center - leg2_delta,
            top_right: center + leg2_delta,
            bottom_right: center + leg_delta,
        }
    }
}

impl TheVillian {
    fn update_center(&mut self) {
        self.center = (self.top_left + self.top_right + self.bottom_left + self.bottom_right) / 4.0;
    }

    fn move_leg(center: Vec2, current_leg: &mut Vec2, direction: Direction) {
        // get the vector between current center and given leg by subtracting center point from leg
        let leg_delta = *current_leg - center;
        let diff = match direction {
            // increase length if direction is extending
            Direction::Extending => leg_delta.normalize(),
            // decrease length if direction is retracting
            Direction::Retracting => -leg_delta.normalize(),
        };
        let leg_delta = leg_delta + diff;
        // calculate new point in absolute space by adding vector to center point
        *current_leg = leg_delta + center;
    }

    fn reset_leg_positions(&mut self) {
        let center = self.center;
        let leg_delta = Vec2::new(50.0, 50.0) * 0.3;
        let leg2_delta = Vec2::new(50.0, -50.0) * 0.3;
        *self = Self {
            center,
            top_left: center - leg_delta,
            bottom_left: center - leg2_delta,
            top_right: center + leg2_delta,
            bottom_right: center + leg_delta,
        }
    }
}

impl QWOPtopus {
    fn new() -> GameResult<QWOPtopus> {
        let s = QWOPtopus {
            input: InputState::default(),
            player: TheVillian::default(),
        };
        Ok(s)
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct InputState {
    top_left: Option<Direction>,
    bottom_left: Option<Direction>,
    top_right: Option<Direction>,
    bottom_right: Option<Direction>,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Extending,
    Retracting,
}

impl event::EventHandler<ggez::GameError> for QWOPtopus {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        dbg!(&self);

        let moved = false;
        let moved = moved
            | self
                .input
                .bottom_right
                .map(|dir| {
                    TheVillian::move_leg(self.player.center, &mut self.player.bottom_right, dir)
                })
                .is_some();
        let moved = moved
            | self
                .input
                .bottom_left
                .map(|dir| {
                    TheVillian::move_leg(self.player.center, &mut self.player.bottom_left, dir)
                })
                .is_some();
        let moved = moved
            | self
                .input
                .top_left
                .map(|dir| TheVillian::move_leg(self.player.center, &mut self.player.top_left, dir))
                .is_some();
        let moved = moved
            | self
                .input
                .top_right
                .map(|dir| {
                    TheVillian::move_leg(self.player.center, &mut self.player.top_right, dir)
                })
                .is_some();
        if !moved {
            self.player.update_center();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let origin = Vec2::new(0.0, 0.0);
        let body = graphics::MeshBuilder::new()
            .circle(graphics::DrawMode::fill(), origin, 20.0, 2.0, Color::WHITE)?
            .line(
                &[origin, self.player.top_left - self.player.center],
                2.0,
                Color::GREEN,
            )?
            .line(
                &[origin, self.player.top_right - self.player.center],
                2.0,
                Color::GREEN,
            )?
            .line(
                &[origin, self.player.bottom_left - self.player.center],
                2.0,
                Color::GREEN,
            )?
            .line(
                &[origin, self.player.bottom_right - self.player.center],
                2.0,
                Color::GREEN,
            )?
            .build(ctx)?;

        graphics::draw(ctx, &body, (self.player.center,))?;

        graphics::present(ctx)?;
        Ok(())
    }

    // Handle key events.  These just map keyboard events
    // and alter our input state appropriately.
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Q => {
                self.input.top_left = Some(Direction::Extending);
            }
            KeyCode::W => {
                self.input.bottom_left = Some(Direction::Extending);
            }
            KeyCode::E => {
                self.input.bottom_left = Some(Direction::Retracting);
            }
            KeyCode::R => {
                self.input.top_left = Some(Direction::Retracting);
            }
            KeyCode::U => {
                self.input.top_right = Some(Direction::Retracting);
            }
            KeyCode::I => {
                self.input.bottom_right = Some(Direction::Retracting);
            }
            KeyCode::O => {
                self.input.bottom_right = Some(Direction::Extending);
            }
            KeyCode::P => {
                self.input.top_right = Some(Direction::Extending);
            }
            KeyCode::Escape => event::quit(ctx),
            KeyCode::Space => self.player.reset_leg_positions(),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Q | KeyCode::R => self.input.top_left = None,
            KeyCode::W | KeyCode::E => self.input.bottom_left = None,
            KeyCode::U | KeyCode::P => self.input.top_right = None,
            KeyCode::I | KeyCode::O => self.input.bottom_right = None,
            _ => (), // Do nothing
        }
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = QWOPtopus::new()?;
    event::run(ctx, event_loop, state)
}
