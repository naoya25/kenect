use dioxus::prelude::*;
use shared::data::{PREFECTURE_DB, TOKYO_DB};
use shared::game::{GamePhase, GameState};
use shared::location::RegionDatabase;

use crate::components::{GameScreen, ResultScreen, SetupScreen};
use crate::utils::random_start;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Prefecture,
    City,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Look,
    NoLook,
}

pub fn db(mode: GameMode) -> &'static RegionDatabase {
    match mode {
        GameMode::Prefecture => &PREFECTURE_DB,
        GameMode::City => &TOKYO_DB,
    }
}

#[derive(Clone)]
pub enum Screen {
    Setup,
    Game(GameState, GameMode, ViewMode),
    Result(GameState, GameMode, ViewMode),
}

const CSS: &str = include_str!("../assets/style.css");

#[component]
pub fn App() -> Element {
    let mut screen = use_signal(|| Screen::Setup);

    let inner = match screen() {
        Screen::Setup => rsx! {
            SetupScreen {
                on_start: move |(names, mode, view_mode)| {
                    let start = random_start(db(mode));
                    screen.set(Screen::Game(GameState::new(start, names, db(mode)), mode, view_mode));
                }
            }
        },
        Screen::Game(state, mode, view_mode) => rsx! {
            GameScreen {
                state: state.clone(),
                mode,
                view_mode,
                on_update: move |new_state: GameState| {
                    if new_state.phase == GamePhase::GameOver {
                        screen.set(Screen::Result(new_state, mode, view_mode));
                    } else {
                        screen.set(Screen::Game(new_state, mode, view_mode));
                    }
                }
            }
        },
        Screen::Result(state, mode, view_mode) => rsx! {
            ResultScreen {
                state: state.clone(),
                mode,
                view_mode,
                on_restart: move |_| screen.set(Screen::Setup),
            }
        },
    };

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1" }
        document::Style { "{CSS}" }
        div { class: "app-header",
            button {
                class: "app-title-btn",
                onclick: move |_| screen.set(Screen::Setup),
                "Kenect"
            }
        }
        {inner}
    }
}
