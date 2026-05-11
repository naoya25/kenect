use dioxus::prelude::*;
use dioxus_web::WebEventExt;
use shared::location::LocationId;
use wasm_bindgen::JsCast;

use crate::app::{GameMode, db};

const JAPAN_SVG: &str = include_str!("../../assets/japan.svg");
const TOKYO_SVG: &str = include_str!("../../assets/tokyo.svg");

#[component]
pub fn LearningScreen(mode: GameMode) -> Element {
    let mut selected: Signal<Option<LocationId>> = use_signal(|| None);
    let db = db(mode);
    let svg = match mode {
        GameMode::Prefecture => JAPAN_SVG,
        GameMode::City => TOKYO_SVG,
    };

    let selected_region = selected().and_then(|id| db.get(id));
    let map_css = learning_map_css(selected(), mode);

    rsx! {
        div { class: "page learning-wrapper",
            div { class: "learning-map-stage",
                style { "{map_css}" }
                div {
                    class: "learning-map-layer",
                    onclick: move |evt: MouseEvent| {
                        let mut next_selected: Option<LocationId> = None;

                        if let Some(web_event) = evt.data().as_ref().try_as_web_event() {
                            if let Some(target) = web_event.target() {
                                if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                                    if let Some(hit) = element.closest("[data-code]").ok().flatten() {
                                        if let Some(code) = hit.get_attribute("data-code") {
                                            if let Ok(raw) = code.parse::<u32>() {
                                                let id = LocationId(raw);
                                                next_selected = if selected() == Some(id) { None } else { Some(id) };
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        selected.set(next_selected);
                    },
                    dangerous_inner_html: "{svg}",
                }
                div { class: "learning-status-panel",
                    div { class: "learning-status-label", "学ぶモード" }
                    div { class: "learning-status-name",
                        if let Some(region) = selected_region {
                            div { class: "learning-status-main", "{region.name}" }
                            div { class: "learning-status-meta", "{region.kana} / {region.roman}" }
                        } else {
                            div { class: "learning-status-main", "白地図をタップ" }
                        }
                    }
                    div { class: "learning-status-help", "地域をタップして名前を表示。もう一度同じ場所をタップすると解除します。" }
                }
            }
        }
    }
}

fn learning_map_css(selected: Option<LocationId>, mode: GameMode) -> String {
    let mut css = String::new();
    let base_selector = match mode {
        GameMode::Prefecture => ".prefecture",
        GameMode::City => ".tokyo-city",
    };

    css.push_str(&format!(
        "{} {{ fill: rgba(5, 5, 20, 0.85) !important; stroke: rgba(0, 245, 255, 0.42) !important; stroke-width: 0.85px !important; cursor: pointer; transition: fill 0.15s ease, filter 0.15s ease, stroke 0.15s ease; }}\n",
        base_selector
    ));

    if let Some(id) = selected {
        css.push_str(&format!(
            "{}[data-code=\"{}\"] {{ fill: rgba(255, 0, 255, 0.6) !important; stroke: #ff00ff !important; stroke-width: 1.5px !important; filter: drop-shadow(0 0 8px #ff00ff) drop-shadow(0 0 18px rgba(255, 0, 255, 0.45)); }}\n",
            base_selector,
            id.0
        ));
    }

    css
}
