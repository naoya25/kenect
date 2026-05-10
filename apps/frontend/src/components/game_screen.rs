use dioxus::prelude::*;
use shared::game::{DeclareError, GameState};

use crate::app::{GameMode, db};
use crate::components::MapView;
use crate::utils::used_names;

#[component]
pub fn GameScreen(state: GameState, mode: GameMode, on_update: EventHandler<GameState>) -> Element {
    let mut input = use_signal(String::new);
    let mut error_msg = use_signal(String::new);

    let current_name = db(mode).name_of(state.current).unwrap_or("");
    let current_hint = db(mode).hint_of(state.current).unwrap_or_default();
    let current_player = state.current_player_index + 1;
    let active_count = state.active_count();
    let move_count = db(mode).valid_move_count(state.current, &state.used);

    let state_snap = state.clone();
    let declare = use_callback(move |_| {
        let name = input();
        let mut new_state = state_snap.clone();
        let candidates = db(mode).find_by_name(&name);
        match new_state.declare(&candidates, db(mode)) {
            Ok(()) => {
                error_msg.set(String::new());
            }
            Err(DeclareError::NotFound) => {
                error_msg.set(format!("「{}」は見つかりません", name));
            }
            Err(DeclareError::NotAdjacent) => {
                error_msg.set(format!("{}は隣接していません", name));
            }
            Err(DeclareError::GameAlreadyOver) => {}
        }
        input.set(String::new());
        on_update.call(new_state);
    });

    rsx! {
        div { class: "game-screen",

            // 地図エリア（上）
            div { class: "map-section",
                MapView { state: state.clone(), mode }
            }

            // コントロールエリア（下）
            div { class: "glass-panel",
                div { class: "panel-grid",

                    // 現在地
                    div { class: "panel-location",
                        div { class: "gc-label", "現在地" }
                        div { class: "gc-name", "{current_name}" }
                        div { class: "gc-hint", "{current_hint}" }
                    }

                    // 統計
                    div { class: "panel-stats",
                        div { class: "stat-chip",
                            div { class: "gc-label", "プレイヤー" }
                            div { class: "gc-stat-val", "{current_player}" }
                        }
                        div { class: "stat-chip",
                            div { class: "gc-label", "残り" }
                            div { class: "gc-stat-val", "{active_count}人" }
                        }
                        div { class: "stat-chip",
                            div { class: "gc-label", "択" }
                            div { class: "gc-stat-val", "{move_count}" }
                        }
                    }

                    // 入力
                    div { class: "panel-input",
                        div { class: "input-row",
                            input {
                                class: "glass-input",
                                r#type: "text",
                                list: "region-candidates",
                                placeholder: "名前を入力",
                                value: "{input}",
                                oninput: move |e| input.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter && !e.is_composing() {
                                        declare(());
                                    }
                                },
                            }
                            button {
                                class: "glass-btn",
                                style: "width: auto; padding: 9px 16px; white-space: nowrap;",
                                onclick: move |_| declare(()),
                                "宣言する"
                            }
                        }
                        datalist {
                            id: "region-candidates",
                            for region in db(mode).all_regions() {
                                option { value: "{region.name}" }
                            }
                        }
                        if !error_msg().is_empty() {
                            div { class: "glass-error", "{error_msg}" }
                        }
                    }
                }

                // 使用済み帯
                div { class: "panel-used-strip",
                    span { class: "gc-label", style: "white-space: nowrap;", "使用済み" }
                    span { class: "panel-used-list",
                        "{used_names(&state, db(mode))}"
                    }
                }
            }
        }
    }
}
