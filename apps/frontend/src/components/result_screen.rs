use dioxus::prelude::*;
use shared::game::GameState;

use crate::app::GameMode;

#[component]
pub fn ResultScreen(state: GameState, mode: GameMode, on_restart: EventHandler<()>) -> Element {
    let ranking = state.ranking();

    rsx! {
        div {
            h1 { "ゲーム終了！" }

            h2 { "スコアランキング" }
            for (rank, &player_index) in ranking.iter().enumerate() {
                p {
                    "第{rank + 1}位 プレイヤー{player_index + 1} Score: {state.players[player_index].score}"
                }
            }

            button {
                onclick: move |_| on_restart.call(()),
                "もう一度遊ぶ"
            }
        }
    }
}
