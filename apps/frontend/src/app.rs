use dioxus::prelude::*;
use shared::data::{CITY_DB, PREFECTURE_DB};
use shared::game::{GamePhase, GameState};
use shared::location::RegionDatabase;

use crate::components::{GameScreen, ResultScreen, SetupScreen};
use crate::utils::random_start;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Prefecture,
    City,
}

pub fn db(mode: GameMode) -> &'static RegionDatabase {
    match mode {
        GameMode::Prefecture => &PREFECTURE_DB,
        GameMode::City => &CITY_DB,
    }
}

#[derive(Clone)]
pub enum Screen {
    Setup,
    Game(GameState, GameMode),
    Result(GameState, GameMode),
}

const CSS: &str = include_str!("../assets/style.css");

#[component]
pub fn App() -> Element {
    let mut screen = use_signal(|| Screen::Setup);

    let inner = match screen() {
        Screen::Setup => rsx! {
            SetupScreen {
                on_start: move |(names, mode)| {
                    let start = random_start(db(mode));
                    screen.set(Screen::Game(GameState::new(start, names, db(mode)), mode));
                }
            }
        },
        Screen::Game(state, mode) => rsx! {
            GameScreen {
                state: state.clone(),
                mode,
                on_update: move |new_state: GameState| {
                    if new_state.phase == GamePhase::GameOver {
                        screen.set(Screen::Result(new_state, mode));
                    } else {
                        screen.set(Screen::Game(new_state, mode));
                    }
                }
            }
        },
        Screen::Result(state, mode) => rsx! {
            ResultScreen {
                state: state.clone(),
                mode,
                on_restart: move |_| screen.set(Screen::Setup),
            }
        },
    };

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Style { "{CSS}" }
        {inner}
    }
}
