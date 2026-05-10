use dioxus::prelude::*;
use shared::game::GameState;

#[component]
pub fn ResultScreen(state: GameState, on_restart: EventHandler<()>) -> Element {
    let ranking = state.ranking();

    rsx! {
        div {
            h1 { "ゲーム終了！" }

            h2 { "スコアランキング" }
            for (rank, &player_index) in ranking.iter().enumerate() {
                p {
                    "第{rank + 1}位　プレイヤー{player_index + 1}　{state.players[player_index].score}県"
                }
            }

            button {
                onclick: move |_| on_restart.call(()),
                "もう一度遊ぶ"
            }
        }
    }
}
