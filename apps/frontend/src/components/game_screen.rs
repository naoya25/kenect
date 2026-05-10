use dioxus::prelude::*;
use shared::data::PREFECTURE_DB;
use shared::game::{DeclareError, GameState};

use crate::utils::used_names;

#[component]
pub fn GameScreen(state: GameState, on_update: EventHandler<GameState>) -> Element {
    let mut input = use_signal(String::new);
    let mut error_msg = use_signal(String::new);

    let current_name = PREFECTURE_DB.name_of(state.current).unwrap_or("");
    let current_hint = PREFECTURE_DB.hint_of(state.current).unwrap_or_default();
    let current_player = state.current_player_index + 1;

    rsx! {
        div {
            h1 { "ケネクト" }
            p { "現在地：{current_name} {current_hint}" }
            p { "プレイヤー{current_player}の番" }
            p { "残りプレイヤー：{state.active_count()}人" }

            input {
                r#type: "text",
                placeholder: "県名を入力（例：神奈川）",
                value: "{input}",
                oninput: move |e| input.set(e.value()),
            }

            button {
                onclick: move |_| {
                    let name = input();
                    let mut new_state = state.clone();
                    let candidates = PREFECTURE_DB.find_by_name(&name);
                    match new_state.declare(&candidates, &PREFECTURE_DB) {
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
                },
                "宣言する"
            }

            if !error_msg().is_empty() {
                p { style: "color: red;", "{error_msg}" }
            }

            h3 { "使用済み" }
            p { "{used_names(&state, &PREFECTURE_DB)}" }
        }
    }
}
