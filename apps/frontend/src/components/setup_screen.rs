use dioxus::prelude::*;

use crate::app::GameMode;

#[component]
pub fn SetupScreen(on_start: EventHandler<(usize, GameMode)>) -> Element {
    let mut player_count = use_signal(|| 2usize);
    let mut mode = use_signal(|| GameMode::Prefecture);

    rsx! {
        div { class: "page setup-wrapper",
            div { class: "setup-card",
                h1 { class: "setup-title", "ケネクト" }
                p { class: "setup-subtitle", "県や市をつないでいくゲームです" }

                fieldset { class: "setup-fieldset",
                    legend { class: "setup-legend", "モード" }
                    div { class: "setup-radio-group",
                        label { class: "setup-radio-label",
                            input {
                                r#type: "radio",
                                name: "mode",
                                checked: mode() == GameMode::Prefecture,
                                oninput: move |_| mode.set(GameMode::Prefecture),
                            }
                            "県モード"
                        }
                        label { class: "setup-radio-label",
                            input {
                                r#type: "radio",
                                name: "mode",
                                checked: mode() == GameMode::City,
                                oninput: move |_| mode.set(GameMode::City),
                            }
                            "市モード（サンプル）"
                        }
                    }
                }

                label { class: "setup-label", "プレイヤー数" }
                input {
                    class: "setup-number-input",
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
                    class: "primary-btn",
                    style: "width: 100%;",
                    onclick: move |_| on_start.call((player_count(), mode())),
                    "ゲーム開始"
                }
            }
        }
    }
}
