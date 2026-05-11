use dioxus::prelude::*;
use shared::game::GameState;
use shared::location::RegionDatabase;

use crate::app::{GameMode, db};

// 新しく追加
const JAPAN_SVG: &str = include_str!("../../assets/japan.svg");
const TOKYO_SVG: &str = include_str!("../../assets/tokyo.svg");

#[component]
pub fn MapView(state: GameState, mode: GameMode) -> Element {
    let svg = match mode {
        GameMode::Prefecture => JAPAN_SVG,
        GameMode::City => TOKYO_SVG, // 東京マップを使用
    };

    let css = region_css(&state, db(mode), mode);

    rsx! {
        style { "{css}" }
        div {
            style: "width: 100%; height: 100%; overflow: hidden;",
            dangerous_inner_html: "{svg}", // 切り替わったSVGを描画
        }
    }
}

fn region_css(state: &GameState, db: &'static RegionDatabase, mode: GameMode) -> String {
    let mut css = String::new();

    // 既存のセレクタ（.prefecture）に加え、東京用のセレクタ（.tokyo-city）も定義
    let base_selector = match mode {
        GameMode::Prefecture => ".prefecture",
        GameMode::City => ".tokyo-city",
    };

    // --- 既存のスタイルを適用（セレクタのみ汎用化） ---

    // デフォルト: 暗い地図 + サイアンの境界
    css.push_str(&format!(
        "{} {{ fill: rgba(5, 5, 20, 0.85) !important; stroke: rgba(0, 245, 255, 0.42) !important; stroke-width: 0.85px !important; }}\n",
        base_selector
    ));

    // 使用済み: 赤系
    for &id in &state.used {
        // JIS X 0402 コード（例：13101）で指定
        css.push_str(&format!(
            "{}[data-code=\"{}\"] {{ fill: rgba(120, 60, 80, 0.7) !important; stroke: rgba(0, 245, 255, 0.26) !important; }}\n",
            base_selector, id.0
        ));
    }

    // 宣言可能: サイアングロー
    for id in db.valid_move_ids(state.current, &state.used) {
        css.push_str(&format!(
            "{}[data-code=\"{}\"] {{ fill: rgba(0, 245, 255, 0.22) !important; stroke: rgba(0, 245, 255, 0.7) !important; stroke-width: 1px !important; }}\n",
            base_selector, id.0
        ));
    }

    // 現在地: マゼンタネオン
    css.push_str(&format!(
        "{}[data-code=\"{}\"] {{ fill: rgba(255, 0, 255, 0.6) !important; stroke: #ff00ff !important; stroke-width: 1.5px !important; filter: drop-shadow(0 0 6px #ff00ff); }}\n",
        base_selector, state.current.0
    ));

    css
}
