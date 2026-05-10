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
        style { "{css}" }
        div {
            style: "width: 100%; height: 100%; overflow: hidden;",
            dangerous_inner_html: "{JAPAN_SVG}",
        }
    }
}

fn prefecture_css(state: &GameState, db: &'static RegionDatabase) -> String {
    let mut css = String::new();

    // デフォルト: 暗い地図 + サイアンの県境
    css.push_str(concat!(
        ".prefecture {",
        " fill: rgba(5, 5, 20, 0.85) !important;",
        " stroke: rgba(0, 245, 255, 0.22) !important;",
        " stroke-width: 0.6px !important;",
        "}\n"
    ));

    // 使用済み: 暗いグレー
    for &id in &state.used {
        css.push_str(&format!(
            ".prefecture[data-code=\"{}\"] {{ fill: rgba(30, 30, 55, 0.9) !important; stroke: rgba(0, 245, 255, 0.12) !important; }}\n",
            id.0
        ));
    }

    // 宣言可能: サイアングロー
    for id in db.valid_move_ids(state.current, &state.used) {
        css.push_str(&format!(
            ".prefecture[data-code=\"{}\"] {{ fill: rgba(0, 245, 255, 0.22) !important; stroke: rgba(0, 245, 255, 0.7) !important; stroke-width: 1px !important; }}\n",
            id.0
        ));
    }

    // 現在地: マゼンタネオン
    css.push_str(&format!(
        ".prefecture[data-code=\"{}\"] {{ fill: rgba(255, 0, 255, 0.6) !important; stroke: #ff00ff !important; stroke-width: 1.5px !important; filter: drop-shadow(0 0 6px #ff00ff); }}\n",
        state.current.0
    ));

    css
}
