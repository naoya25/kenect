use dioxus::prelude::*;
use shared::game::GameState;

use crate::app::GameMode;

#[component]
pub fn ResultScreen(state: GameState, mode: GameMode, on_restart: EventHandler<()>) -> Element {
    let ranking = state.ranking();
    let score_unit = match mode {
        GameMode::Prefecture => "県",
        GameMode::City => "市",
    };
    let medals = ["🥇", "🥈", "🥉"];

    rsx! {
        div { class: "page result-wrapper",
            div { class: "result-card",
                h1 { class: "result-title", "ゲーム終了！" }

                div { class: "ranking-list",
                    for (rank, &player_index) in ranking.iter().enumerate() {
                        div { class: "ranking-item",
                            span { class: "ranking-medal",
                                "{medals.get(rank).copied().unwrap_or(\"　\")}"
                            }
                            span { class: "ranking-player",
                                "{state.players[player_index].name}"
                            }
                            span { class: "ranking-score",
                                "{state.players[player_index].score} {score_unit}"
                            }
                        }
                    }
                }

                button {
                    class: "primary-btn",
                    onclick: move |_| on_restart.call(()),
                    "もう一度遊ぶ"
                }
            }
        }
    }
}
