use dioxus::prelude::*;
use gloo_timers::future::sleep;
use shared::game::{DeclareError, GameState};

use crate::app::{GameMode, HintMode, ViewMode, db};
use crate::components::MapView;
use crate::utils::used_names;

#[component]
pub fn GameScreen(
    state: GameState,
    mode: GameMode,
    view_mode: ViewMode,
    hint_mode: HintMode,
    on_update: EventHandler<GameState>,
) -> Element {
    // query: ユーザーが実際に打った文字（フィルタリングに使う）
    // input: テキストフィールドの表示値（矢印キーで候補名に書き換わる）
    let mut query = use_signal(String::new);
    let mut input = use_signal(String::new);
    let mut error_msg = use_signal(String::new);
    let mut show_dropdown = use_signal(|| false);
    let mut highlight_index: Signal<Option<usize>> = use_signal(|| None);

    // ターン変更エフェクト用
    let mut last_player_index = use_signal(|| state.current_player_index);
    let mut last_wrong_count = use_signal(|| state.players[state.current_player_index].wrong_count);
    let mut show_turn_effect = use_signal(|| false);
    let mut effect_player_name = use_signal(String::new);
    let mut effect_message = use_signal(String::new);

    let current_name = db(mode).name_of(state.current).unwrap_or("");
    let current_hint = db(mode).hint_of(state.current).unwrap_or_default();
    let current_player_name = state.players[state.current_player_index].name.clone();
    let active_count = state.active_count();
    let move_count = db(mode).valid_move_count(state.current, &state.used);
    let miss_limit = if hint_mode == HintMode::NoHint { 1 } else { 3 };

    // ターン変更を検出
    let current_wrong_count = state.players[state.current_player_index].wrong_count;
    let player_changed = last_player_index() != state.current_player_index;
    let wrong_count_changed = last_wrong_count() != current_wrong_count;

    if player_changed || wrong_count_changed {
        let previous_player_index = last_player_index();
        last_player_index.set(state.current_player_index);
        last_wrong_count.set(current_wrong_count);

        if current_wrong_count > 0 && !player_changed {
            effect_message.set(format!("{}ミス", current_wrong_count));
        } else if player_changed {
            let previous_wrong_count = state.players[previous_player_index].wrong_count;
            if previous_wrong_count >= miss_limit {
                effect_message.set(format!("{}ミス: チェンジ", previous_wrong_count));
            } else if previous_wrong_count > 0 {
                effect_message.set(format!("{}ミス", previous_wrong_count));
            } else {
                effect_player_name.set(current_player_name.clone());
                effect_message.set(format!("{}のターン", current_player_name));
            }
        } else {
            effect_message.set(String::new());
        }
        show_turn_effect.set(true);

        // 2秒後に非表示
        dioxus::prelude::spawn({
            let mut effect_signal = show_turn_effect;
            async move {
                sleep(std::time::Duration::from_secs(2)).await;
                effect_signal.set(false);
            }
        });
    }
    // フィルタリングは query ベース（矢印キー操作で変わらない）
    let filtered: Vec<&'static str> = {
        let q = query();
        if q.is_empty() {
            vec![]
        } else {
            let q_lower = q.to_lowercase();
            db(mode)
                .all_regions()
                .iter()
                .filter(|r| {
                    r.name.contains(q.as_str())
                        || r.kana.contains(q.as_str())
                        || r.roman.to_lowercase().contains(&q_lower)
                })
                .map(|r| r.name)
                .take(8)
                .collect()
        }
    };

    let state_snap = state.clone();
    let declare = use_callback(move |_| {
        let name = input();
        let mut new_state = state_snap.clone();
        let candidates = db(mode).find_by_name(&name);
        match new_state.declare_with_miss_limit(&candidates, db(mode), miss_limit) {
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
        query.set(String::new());
        input.set(String::new());
        show_dropdown.set(false);
        highlight_index.set(None);
        on_update.call(new_state);
    });

    rsx! {
        div { class: "game-screen",

            // 地図エリア（上）
            div { class: "map-section",
                if view_mode == ViewMode::Look {
                    MapView { state: state.clone(), mode }
                }
                div { class: "game-title-badge", "Kenect" }

                // ターン変更エフェクト
                if show_turn_effect() {
                    div { class: "turn-change-effect",
                        div { class: "turn-change-text",
                            if effect_message().is_empty() {
                                "{effect_player_name}のターン"
                            } else {
                                "{effect_message}"
                            }
                        }
                    }
                }
            }

            // コントロールエリア（下）
            div { class: "glass-panel",
                div { class: "panel-grid",

                    // 現在地
                    div { class: "panel-location",
                        div { class: "gc-label", "現在地" }
                        div { class: "gc-name", "{current_name}" }
                        if hint_mode == HintMode::Normal {
                            div { class: "gc-hint", "{current_hint}" }
                        }
                    }

                    // 統計
                    div { class: "panel-stats",
                        div { class: "stat-chip",
                            div { class: "gc-label", "プレイヤー" }
                            div { class: "gc-stat-val", "{current_player_name}" }
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
                            div { class: "input-autocomplete",
                                input {
                                    class: "glass-input",
                                    r#type: "text",
                                    placeholder: "名前を入力",
                                    value: "{input}",
                                    autocomplete: "off",
                                    oninput: move |e| {
                                        let v = e.value();
                                        // 両方更新：打った文字がそのままクエリになる
                                        query.set(v.clone());
                                        input.set(v.clone());
                                        highlight_index.set(None);
                                        show_dropdown.set(!v.is_empty());
                                    },
                                    onfocus: move |_| {
                                        if !query().is_empty() {
                                            show_dropdown.set(true);
                                        }
                                    },
                                    onblur: move |_| {
                                        show_dropdown.set(false);
                                    },
                                    onkeydown: move |e| {
                                        if e.is_composing() { return; }
                                        match e.key() {
                                            Key::Enter => {
                                                declare(());
                                            }
                                            Key::ArrowDown => {
                                                let len = filtered.len();
                                                if len == 0 { return; }
                                                e.prevent_default();
                                                let next = match highlight_index() {
                                                    None => 0,
                                                    Some(i) => (i + 1).min(len - 1),
                                                };
                                                highlight_index.set(Some(next));
                                                // input の表示だけ候補名に書き換える（query は変えない）
                                                if let Some(name) = filtered.get(next) {
                                                    input.set(name.to_string());
                                                }
                                            }
                                            Key::ArrowUp => {
                                                let len = filtered.len();
                                                if len == 0 { return; }
                                                e.prevent_default();
                                                let next = match highlight_index() {
                                                    None | Some(0) => 0,
                                                    Some(i) => i - 1,
                                                };
                                                highlight_index.set(Some(next));
                                                if let Some(name) = filtered.get(next) {
                                                    input.set(name.to_string());
                                                }
                                            }
                                            Key::Escape => {
                                                // Escape で選択解除し、元の打った文字に戻す
                                                show_dropdown.set(false);
                                                highlight_index.set(None);
                                                input.set(query());
                                            }
                                            _ => {}
                                        }
                                    },
                                }
                                if show_dropdown() && !filtered.is_empty() {
                                    div { class: "cyber-dropdown",
                                        for (idx, name) in filtered.iter().enumerate() {
                                            {
                                                let name_str = *name;
                                                let is_highlighted = highlight_index() == Some(idx);
                                                let cls = if is_highlighted {
                                                    "cyber-dropdown-item cyber-dropdown-item--active"
                                                } else {
                                                    "cyber-dropdown-item"
                                                };
                                                rsx! {
                                                    div {
                                                        class: "{cls}",
                                                        onmousedown: move |_| {
                                                            input.set(name_str.to_string());
                                                            query.set(name_str.to_string());
                                                            show_dropdown.set(false);
                                                            highlight_index.set(None);
                                                        },
                                                        "{name_str}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            button {
                                class: "glass-btn",
                                style: "width: auto; padding: 9px 16px; white-space: nowrap;",
                                onclick: move |_| declare(()),
                                "宣言する"
                            }
                        }
                        if !error_msg().is_empty() {
                            div { class: "glass-error", "{error_msg}" }
                        }
                    }
                }

                // 使用済み帯
                div { class: "panel-used-strip",
                    span { class: "gc-label", style: "white-space: nowrap;", "経路" }
                    span { class: "panel-used-list",
                        "{used_names(&state, db(mode))}"
                    }
                }
            }
        }
    }
}
