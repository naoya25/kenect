use dioxus::prelude::*;

use crate::app::GameMode;

#[component]
pub fn SetupScreen(on_start: EventHandler<(usize, GameMode)>) -> Element {
    let mut player_count = use_signal(|| 2usize);
    let mut mode = use_signal(|| GameMode::Prefecture);

    rsx! {
        div {
            h1 { "ケネクト" }
            p { "県や市をつないでいくゲームです" }

            fieldset {
                legend { "モード" }
                label {
                    input {
                        r#type: "radio",
                        name: "mode",
                        checked: mode() == GameMode::Prefecture,
                        oninput: move |_| mode.set(GameMode::Prefecture),
                    }
                    "県モード"
                }
            }

            label { "プレイヤー数：" }
            input {
                r#type: "number",
                min: "1",
                max: "10",
                value: "{player_count}",
                oninput: move |e| {
                    if let Ok(n) = e.value().parse::<usize>()
                        && n >= 1 { player_count.set(n); }
                }
            }

            button {
                onclick: move |_| on_start.call((player_count(), mode())),
                "ゲーム開始"
            }
        }
    }
}
