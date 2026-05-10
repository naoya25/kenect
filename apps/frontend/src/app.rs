use dioxus::prelude::*;
use shared::game::{GamePhase, GameState};

use crate::components::{GameScreen, ResultScreen, SetupScreen};
use crate::utils::random_start;

#[derive(Clone)]
pub enum Screen {
    Setup,
    Game(GameState),
    Result(GameState),
}

#[component]
pub fn App() -> Element {
    let mut screen = use_signal(|| Screen::Setup);

    match screen() {
        Screen::Setup => rsx! {
            SetupScreen { on_start: move |player_count| {
                let start = random_start();
                screen.set(Screen::Game(GameState::new(start, player_count)));
            }}
        },
        Screen::Game(state) => rsx! {
            GameScreen {
                state: state.clone(),
                on_update: move |new_state: GameState| {
                    if new_state.phase == GamePhase::GameOver {
                        screen.set(Screen::Result(new_state));
                    } else {
                        screen.set(Screen::Game(new_state));
                    }
                }
            }
        },
        Screen::Result(state) => rsx! {
            ResultScreen {
                state: state.clone(),
                on_restart: move |_| screen.set(Screen::Setup),
            }
        },
    }
}
