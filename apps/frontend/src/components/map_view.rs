use dioxus::prelude::*;
use rand::random_range;
use shared::game::GameState;
use shared::location::RegionDatabase;

use crate::app::{GameMode, db};

// SVG assets
const JAPAN_SVG: &str = include_str!("../../assets/japan.svg");
const TOKYO_SVG: &str = include_str!("../../assets/tokyo.svg");

#[component]
pub fn MapView(state: GameState, mode: GameMode) -> Element {
    let db = db(mode);
    let svg = match mode {
        GameMode::Prefecture => JAPAN_SVG,
        GameMode::City => TOKYO_SVG,
    };

    let css = region_css(&state, db, mode);
    let svg_with_hints = add_hints(svg, &state, db);

    rsx! {
        style { "{css}" }
        div {
            style: "width: 100%; height: 100%; overflow: hidden;",
            dangerous_inner_html: "{svg_with_hints}",
        }
    }
}

fn add_hints(svg: &str, state: &GameState, db: &'static RegionDatabase) -> String {
    let current_player = &state.players[state.current_player_index];
    if current_player.wrong_count == 0 {
        return svg.to_string();
    }

    let hint_level = current_player.wrong_count;
    let candidates = db.valid_move_ids(state.current, &state.used);
    if candidates.is_empty() {
        return svg.to_string();
    }

    let id = candidates[random_range(0..candidates.len())];
    let needle = format!("data-code=\"{}\"", id.0);
    let Some(group_start) = svg.find(&needle).and_then(|pos| svg[..pos].rfind("<g")) else {
        return svg.to_string();
    };

    let Some(group_end_start) = svg[group_start..].find("</g>") else {
        return svg.to_string();
    };
    let group_end = group_start + group_end_start;
    let group = &svg[group_start..group_end];

    let Some((x, y)) = group_center(group) else {
        return svg.to_string();
    };

    let label = hint_label(db, id, hint_level);
    let insert = format!(
        "<g class=\"map-hint-group\"><rect class=\"map-hint-bg\" x=\"{:.2}\" y=\"{:.2}\" width=\"22\" height=\"22\" rx=\"8\" ry=\"8\" /><text class=\"map-hint\" x=\"{:.2}\" y=\"{:.2}\">{}</text></g>",
        x - 11.0,
        y - 11.0,
        x,
        y,
        label
    );

    let mut result = svg.to_string();
    result.insert_str(group_end, &insert);
    result
}

fn hint_label(
    db: &'static RegionDatabase,
    id: shared::location::LocationId,
    hint_level: u8,
) -> String {
    let region = db.get(id);
    let ch = match hint_level {
        1 => region.and_then(|r| r.roman.chars().next()),
        _ => region.and_then(|r| r.kana.chars().next()),
    }
    .unwrap_or('?');

    ch.to_string()
}

fn group_center(group: &str) -> Option<(f64, f64)> {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut found_any = false;

    let mut search_start = 0;
    while let Some(points_pos) = group[search_start..].find("points=\"") {
        let start = search_start + points_pos + "points=\"".len();
        let Some(end_rel) = group[start..].find('"') else {
            break;
        };
        let points = &group[start..start + end_rel];
        let numbers = parse_numbers(points);
        update_bounds(
            &numbers,
            &mut min_x,
            &mut min_y,
            &mut max_x,
            &mut max_y,
            &mut found_any,
        );
        search_start = start + end_rel + 1;
    }

    if !found_any {
        let mut search_start = 0;
        while let Some(path_pos) = group[search_start..].find("d=\"") {
            let start = search_start + path_pos + "d=\"".len();
            let Some(end_rel) = group[start..].find('"') else {
                break;
            };
            let path = &group[start..start + end_rel];
            let numbers = parse_numbers(path);
            update_bounds(
                &numbers,
                &mut min_x,
                &mut min_y,
                &mut max_x,
                &mut max_y,
                &mut found_any,
            );
            search_start = start + end_rel + 1;
        }
    }

    if found_any {
        Some(((min_x + max_x) / 2.0, (min_y + max_y) / 2.0))
    } else {
        None
    }
}

fn update_bounds(
    numbers: &[f64],
    min_x: &mut f64,
    min_y: &mut f64,
    max_x: &mut f64,
    max_y: &mut f64,
    found_any: &mut bool,
) {
    let mut index = 0;
    while index + 1 < numbers.len() {
        let x = numbers[index];
        let y = numbers[index + 1];
        *found_any = true;
        *min_x = min_x.min(x);
        *min_y = min_y.min(y);
        *max_x = max_x.max(x);
        *max_y = max_y.max(y);
        index += 2;
    }
}

fn parse_numbers(input: &str) -> Vec<f64> {
    input
        .split(|c: char| !(c.is_ascii_digit() || matches!(c, '.' | '-' | '+' | 'e' | 'E')))
        .filter_map(|part| {
            if part.is_empty() {
                None
            } else {
                part.parse::<f64>().ok()
            }
        })
        .collect()
}

fn region_css(state: &GameState, db: &'static RegionDatabase, mode: GameMode) -> String {
    let mut css = String::new();

    let base_selector = match mode {
        GameMode::Prefecture => ".prefecture",
        GameMode::City => ".tokyo-city",
    };

    css.push_str(&format!(
        "{} {{ fill: rgba(5, 5, 20, 0.85) !important; stroke: rgba(0, 245, 255, 0.42) !important; stroke-width: 0.85px !important; }}\n",
        base_selector
    ));

    for &id in &state.used {
        css.push_str(&format!(
            "{}[data-code=\"{}\"] {{ fill: rgba(120, 60, 80, 0.7) !important; stroke: rgba(0, 245, 255, 0.26) !important; }}\n",
            base_selector, id.0
        ));
    }

    for id in db.valid_move_ids(state.current, &state.used) {
        css.push_str(&format!(
            "{}[data-code=\"{}\"] {{ fill: rgba(0, 245, 255, 0.22) !important; stroke: rgba(0, 245, 255, 0.7) !important; stroke-width: 1px !important; }}\n",
            base_selector, id.0
        ));
    }

    css.push_str(&format!(
        "{}[data-code=\"{}\"] {{ fill: rgba(255, 0, 255, 0.6) !important; stroke: #ff00ff !important; stroke-width: 1.5px !important; filter: drop-shadow(0 0 6px #ff00ff); }}\n",
        base_selector, state.current.0
    ));

    css.push_str(
        ".map-hint { font-family: 'Share Tech Mono', monospace; font-size: 14px; fill: #00f5ff; stroke: #00000033; paint-order: stroke; text-anchor: middle; dominant-baseline: middle; pointer-events: none; }",
    );

    css.push_str(
        ".map-hint-bg { fill: rgba(0, 0, 0, 0.78); filter: blur(1.5px); pointer-events: none; }",
    );

    css
}
