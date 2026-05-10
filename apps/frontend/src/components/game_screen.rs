use dioxus::prelude::*;
use shared::game::{DeclareError, GameState};

use crate::app::{GameMode, db};
use crate::utils::used_names;

#[component]
pub fn GameScreen(state: GameState, mode: GameMode, on_update: EventHandler<GameState>) -> Element {
    let mut input = use_signal(String::new);
    let mut error_msg = use_signal(String::new);

    let current_name = db(mode).name_of(state.current).unwrap_or("");
    let current_hint = db(mode).hint_of(state.current).unwrap_or_default();
    let current_player = state.current_player_index + 1;

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
        div {
            h1 { "ケネクト" }
            p { "現在地：{current_name} {current_hint}" }
            p { "プレイヤー{current_player}の番" }
            p { "残りプレイヤー：{state.active_count()}人" }

            input {
                r#type: "text",
                list: "region-candidates",
                placeholder: "名前を入力",
                value: "{input}",
                oninput: move |e| input.set(e.value()),
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        declare(());
                    }
                },
            }
            datalist {
                id: "region-candidates",
                for region in db(mode).all_regions() {
                    option { value: "{region.name}" }
                }
            }

            button {
                onclick: move |_| declare(()),
                "宣言する"
            }

            if !error_msg().is_empty() {
                p { style: "color: red;", "{error_msg}" }
            }

            h3 { "使用済み" }
            p { "{used_names(&state, db(mode))}" }
        }
    }
}
