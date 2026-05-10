use dioxus::prelude::*;
use shared::game::GameState;
use shared::location::RegionDatabase;

use crate::app::{GameMode, db};

const JAPAN_SVG: &str = include_str!("../../assets/japan.svg");

#[component]
pub fn MapView(state: GameState, mode: GameMode) -> Element {
    let css = match mode {
        GameMode::Prefecture => prefecture_css(&state, db(mode)),
        GameMode::City => String::new(),
    };

    rsx! {
        div {
            style { "{css}" }
            div {
                style: "width: 100%; max-width: 560px; margin: 0 auto;",
                dangerous_inner_html: "{JAPAN_SVG}",
            }
        }
    }
}

fn prefecture_css(state: &GameState, db: &'static RegionDatabase) -> String {
    let mut css = String::new();

    // 使用済みをグレーに
    for &id in &state.used {
        css.push_str(&format!(
            ".prefecture[data-code=\"{}\"] {{ fill: #95A5A6 !important; }}\n",
            id.0
        ));
    }

    // 宣言可能な隣接県を薄い青に
    for id in db.valid_move_ids(state.current, &state.used) {
        css.push_str(&format!(
            ".prefecture[data-code=\"{}\"] {{ fill: #AED6F1 !important; }}\n",
            id.0
        ));
    }

    // 現在地を赤に（最後に上書き）
    css.push_str(&format!(
        ".prefecture[data-code=\"{}\"] {{ fill: #E74C3C !important; }}\n",
        state.current.0
    ));

    css
}
