use dioxus::prelude::*;

use crate::app::{GameMode, HintMode, ViewMode};

#[component]
pub fn SetupScreen(on_start: EventHandler<(Vec<String>, GameMode, ViewMode, HintMode)>) -> Element {
    let mut player_count = use_signal(|| 2usize);
    let mut mode = use_signal(|| GameMode::Prefecture);
    let mut view_mode = use_signal(|| ViewMode::Look);
    let mut hint_mode = use_signal(|| HintMode::Normal);
    let mut names: Signal<Vec<String>> = use_signal(|| vec![String::new(), String::new()]);

    rsx! {
        div { class: "page setup-wrapper",
            div { class: "setup-card",
                p { class: "setup-subtitle", "県や東京の自治体をつないでいくゲームです" }

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
                            "東京モード"
                        }
                    }
                }

                fieldset { class: "setup-fieldset",
                    legend { class: "setup-legend", "表示" }
                    div { class: "setup-radio-group",
                        label { class: "setup-radio-label",
                            input {
                                r#type: "radio",
                                name: "view_mode",
                                checked: view_mode() == ViewMode::Look,
                                oninput: move |_| view_mode.set(ViewMode::Look),
                            }
                            "ルックモード"
                        }
                        label { class: "setup-radio-label",
                            input {
                                r#type: "radio",
                                name: "view_mode",
                                checked: view_mode() == ViewMode::NoLook,
                                oninput: move |_| view_mode.set(ViewMode::NoLook),
                            }
                            "ノールックモード"
                        }
                    }
                }

                fieldset { class: "setup-fieldset",
                    legend { class: "setup-legend", "ヒント" }
                    div { class: "setup-radio-group",
                        label { class: "setup-radio-label",
                            input {
                                r#type: "radio",
                                name: "hint_mode",
                                checked: hint_mode() == HintMode::Normal,
                                oninput: move |_| hint_mode.set(HintMode::Normal),
                            }
                            "通常ヒント"
                        }
                        label { class: "setup-radio-label",
                            input {
                                r#type: "radio",
                                name: "hint_mode",
                                checked: hint_mode() == HintMode::NoHint,
                                oninput: move |_| hint_mode.set(HintMode::NoHint),
                            }
                            "ノーヒントモード"
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
                            && n >= 1
                        {
                            player_count.set(n);
                            names.with_mut(|ns| ns.resize(n, String::new()));
                        }
                    }
                }

                fieldset { class: "setup-fieldset",
                    legend { class: "setup-legend", "プレイヤー名（省略可）" }
                    div { class: "setup-names-list",
                        for i in 0..player_count() {
                            input {
                                class: "glass-input setup-name-input",
                                r#type: "text",
                                placeholder: "プレイヤー{i + 1}",
                                value: "{names.read().get(i).cloned().unwrap_or_default()}",
                                oninput: move |e| {
                                    names.with_mut(|ns| {
                                        if ns.len() > i {
                                            ns[i] = e.value();
                                        }
                                    });
                                },
                            }
                        }
                    }
                }

                button {
                    class: "primary-btn",
                    style: "width: 100%;",
                    onclick: move |_| {
                        let resolved: Vec<String> = (0..player_count())
                            .map(|i| {
                                let n = names.read().get(i).cloned().unwrap_or_default();
                                let trimmed = n.trim().to_string();
                                if trimmed.is_empty() {
                                    format!("プレイヤー{}", i + 1)
                                } else {
                                    trimmed
                                }
                            })
                            .collect();
                                on_start.call((resolved, mode(), view_mode(), hint_mode()));
                    },
                    "ゲーム開始"
                }
            }
        }
    }
}
