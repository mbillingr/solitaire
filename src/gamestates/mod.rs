mod giveup_state;
mod main_state;
mod victory_state;
mod welcome_state;

use std::fmt;

use ggez::{Context, GameResult};
use ggez::event;
use sdl2::event::EventType;

use self::welcome_state::WelcomeState;
use self::main_state::MainState;
use self::victory_state::VictoryState;
use self::giveup_state::GiveupState;

pub enum GameWrapper {
    Welcome(WelcomeState),
    Game(MainState),
    Victory(VictoryState),
    GiveUp(GiveupState),
    Quit,
}

use self::GameWrapper::*;

impl GameWrapper {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(GameWrapper::Welcome(WelcomeState::new(ctx)?))
    }

    pub fn run(self, ctx: &mut Context) -> GameResult<Self> {
        // make sure no unhandled events are left when entering a new state
        ctx.event_context.flush_events(EventType::First as u32, EventType::Last as u32);
        info!("Entering game state {}", self);
        match self {
            Welcome(mut state) => {
                event::run(ctx, &mut state)?;
                Ok(state.next_state())
            },
            Game(mut state) => {
                event::run(ctx, &mut state)?;
                Ok(state.next_state())
            },
            Victory(mut state) => {
                event::run(ctx, &mut state)?;
                Ok(state.next_state())
            },
            GiveUp(mut state) => {
                event::run(ctx, &mut state)?;
                Ok(state.next_state())
            },
            Quit => panic!("Invalid Game State")
        }
    }
}

impl fmt::Display for GameWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Welcome(_) => "Welcome",
            Game(_) => "Game",
            Victory(_) => "Victory",
            GiveUp(_) => "GiveUp",
            Quit => "Quit",
        };
        write!(f, "{}", name)
    }
}
