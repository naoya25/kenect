use dioxus::prelude::*;
use shared::game::{DeclareError, GameState};
use shared::prefecture::get_prefecture;

use crate::utils::{find_prefecture_by_name, used_names};

#[component]
pub fn GameScreen(state: GameState, on_update: EventHandler<GameState>) -> Element {
    let mut input = use_signal(String::new);
    let mut error_msg = use_signal(String::new);

    let current_pref = get_prefecture(state.current);
    let current_name = current_pref.map(|p| p.name).unwrap_or("");
    let current_hint = current_pref
        .map(|p| format!("{} / {}", p.kana, p.roman))
        .unwrap_or_default();
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

                    let candidates = find_prefecture_by_name(&name);
                    match new_state.declare(&candidates) {
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
            p { "{used_names(&state)}" }
        }
    }
}
