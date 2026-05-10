use dioxus::prelude::*;
use shared::game::GameState;

use crate::app::{GameMode, db};

/// パーセンテージに応じた (color, text-shadow) を返す
fn conquest_style(pct: u32) -> (&'static str, &'static str) {
    match pct {
        0..=30 => ("#4a7090", "none"),
        31..=55 => (
            "#00f5ff",
            "0 0 6px rgba(0,245,255,0.7), 0 0 14px rgba(0,245,255,0.35)",
        ),
        56..=80 => (
            "#ff00ff",
            "0 0 8px rgba(255,0,255,0.8), 0 0 18px rgba(255,0,255,0.4)",
        ),
        _ => (
            "#ffd700",
            "0 0 10px rgba(255,215,0,0.9), 0 0 24px rgba(255,215,0,0.5), 0 0 40px rgba(255,215,0,0.2)",
        ),
    }
}

fn bar_color(pct: u32) -> &'static str {
    match pct {
        0..=30 => "rgba(74, 112, 144, 0.6)",
        31..=55 => "rgba(0, 245, 255, 0.7)",
        56..=80 => "rgba(255, 0, 255, 0.7)",
        _ => "rgba(255, 215, 0, 0.8)",
    }
}

#[component]
pub fn ResultScreen(state: GameState, mode: GameMode, on_restart: EventHandler<()>) -> Element {
    let ranking = state.ranking();
    let score_unit = match mode {
        GameMode::Prefecture => "県",
        GameMode::City => "市区町村",
    };
    let medals = ["🥇", "🥈", "🥉"];

    let total = db(mode).all_regions().len();
    let overall_count = state.used.len();
    let overall_pct = (overall_count as f64 / total as f64 * 100.0).round() as u32;
    let (overall_color, overall_glow) = conquest_style(overall_pct);

    rsx! {
        div { class: "page result-wrapper",
            div { class: "result-card",
                h1 { class: "result-title", "ゲーム終了！" }

                // ランキング
                div { class: "ranking-list",
                    for (rank, &player_index) in ranking.iter().enumerate() {
                        div { class: "ranking-item",
                            span { class: "ranking-medal",
                                "{medals.get(rank).copied().unwrap_or(\"　\")}"
                            }
                            span { class: "ranking-player",
                                "{state.players[player_index].name}"
                            }
                            span { class: "ranking-score",
                                "{state.players[player_index].score} {score_unit}"
                            }
                        }
                    }
                }

                // 制覇率セクション
                div { class: "conquest-section",
                    div { class: "conquest-section-title", "制覇率" }

                    // 全体制覇率
                    div { class: "conquest-overall",
                        div { class: "conquest-overall-label", "全体制覇率" }
                        div { class: "conquest-bar-track",
                            div {
                                class: "conquest-bar-fill",
                                style: "width: {overall_pct}%; background: {bar_color(overall_pct)};",
                            }
                        }
                        div { class: "conquest-overall-nums",
                            span {
                                class: "conquest-overall-pct",
                                style: "color: {overall_color}; text-shadow: {overall_glow};",
                                "{overall_pct}%"
                            }
                            span { class: "conquest-overall-count",
                                "{overall_count} / {total} {score_unit}"
                            }
                        }
                    }

                    // プレイヤー別
                    div { class: "conquest-players",
                        for &player_index in &ranking {
                            {
                                let player = &state.players[player_index];
                                let pct = (player.score as f64 / total as f64 * 100.0).round() as u32;
                                let (color, glow) = conquest_style(pct);
                                rsx! {
                                    div { class: "conquest-player-row",
                                        span { class: "conquest-player-name",
                                            "{player.name}"
                                        }
                                        div { class: "conquest-bar-track conquest-bar-track--sm",
                                            div {
                                                class: "conquest-bar-fill",
                                                style: "width: {pct}%; background: {bar_color(pct)};",
                                            }
                                        }
                                        span {
                                            class: "conquest-player-pct",
                                            style: "color: {color}; text-shadow: {glow};",
                                            "{pct}%"
                                        }
                                        span { class: "conquest-player-count",
                                            "{player.score}{score_unit}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                button {
                    class: "primary-btn",
                    style: "width: 100%; margin-top: 8px;",
                    onclick: move |_| on_restart.call(()),
                    "もう一度遊ぶ"
                }
            }
        }
    }
}
