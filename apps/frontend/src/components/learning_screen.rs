use dioxus::prelude::*;
use dioxus_web::WebEventExt;
use std::collections::HashMap;
use shared::location::LocationId;
use wasm_bindgen::JsCast;

use crate::app::{GameMode, db};

const JAPAN_SVG: &str = include_str!("../../assets/japan.svg");
const TOKYO_SVG: &str = include_str!("../../assets/tokyo.svg");

#[component]
pub fn LearningScreen(mode: GameMode) -> Element {
    let mut selected: Signal<Option<LocationId>> = use_signal(|| None);
    let mut zoom: Signal<f64> = use_signal(|| 1.0);
    let mut pan_x: Signal<f64> = use_signal(|| 0.0);
    let mut pan_y: Signal<f64> = use_signal(|| 0.0);
    let mut active_pointers: Signal<HashMap<i32, (f64, f64)>> = use_signal(HashMap::new);
    let mut drag_start_pointer: Signal<Option<(f64, f64)>> = use_signal(|| None);
    let mut drag_start_pan: Signal<(f64, f64)> = use_signal(|| (0.0, 0.0));
    let mut drag_moved: Signal<bool> = use_signal(|| false);
    let mut pinch_start_distance: Signal<Option<f64>> = use_signal(|| None);
    let mut pinch_start_zoom: Signal<f64> = use_signal(|| 1.0);
    let db = db(mode);
    let svg = match mode {
        GameMode::Prefecture => JAPAN_SVG,
        GameMode::City => TOKYO_SVG,
    };

    let selected_region = selected().and_then(|id| db.get(id));
    let map_css = learning_map_css(selected(), mode);
    let map_transform = format!(
        "transform: translate3d({}px, {}px, 0) scale({});",
        pan_x(),
        pan_y(),
        zoom()
    );

    rsx! {
        div { class: "page learning-wrapper",
            div { class: "learning-map-stage",
                style { "{map_css}" }
                div {
                    class: "learning-map-layer",
                    style: "{map_transform}",
                    onwheel: move |evt| {
                        evt.prevent_default();
                        let delta = evt.data().delta().strip_units().y;
                        let current = zoom();
                        let next = if delta < 0.0 { current * 1.08 } else { current / 1.08 };
                        zoom.set(next.clamp(1.0, 4.0));
                    },
                    onpointerdown: move |evt| {
                        let coords = evt.data().coordinates().client();
                        let pointer_id = evt.data().pointer_id();
                        let has_pointers = active_pointers.read().len();
                        active_pointers.with_mut(|pointers| {
                            pointers.insert(pointer_id, (coords.x, coords.y));
                        });
                        if has_pointers == 0 {
                            drag_start_pointer.set(Some((coords.x, coords.y)));
                            drag_start_pan.set((pan_x(), pan_y()));
                            drag_moved.set(false);
                        } else if has_pointers == 1 {
                            drag_start_pointer.set(None);
                            pinch_start_zoom.set(zoom());
                            let pointers = active_pointers.read();
                            let mut values = pointers.values();
                            if let (Some((x1, y1)), Some((x2, y2))) =
                                (values.next().copied(), values.next().copied())
                            {
                                pinch_start_distance
                                    .set(Some(((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()));
                            }
                        } else {
                            drag_start_pointer.set(None);
                        }
                    },
                    onpointermove: move |evt| {
                        let pointer_id = evt.data().pointer_id();
                        let coords = evt.data().coordinates().client();
                        let x = coords.x;
                        let y = coords.y;

                        active_pointers.with_mut(|pointers| {
                            if let Some(pointer) = pointers.get_mut(&pointer_id) {
                                *pointer = (x, y);
                            }

                            if pointers.len() == 1
                                && let Some((start_x, start_y)) = drag_start_pointer()
                                && start_x.is_finite()
                                && start_y.is_finite()
                                && Some(pointer_id).is_some()
                            {
                                let dx = x - start_x;
                                let dy = y - start_y;
                                if dx.abs() > 0.5 || dy.abs() > 0.5 {
                                    drag_moved.set(true);
                                }
                                let (start_pan_x, start_pan_y) = drag_start_pan();
                                pan_x.set(start_pan_x + dx);
                                pan_y.set(start_pan_y + dy);
                            }

                            if pointers.len() == 2 {
                                let mut values = pointers.values();
                                let Some((x1, y1)) = values.next().copied() else { return; };
                                let Some((x2, y2)) = values.next().copied() else { return; };
                                let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
                                if let Some(start_distance) = pinch_start_distance() {
                                    if start_distance > 0.0 {
                                        let next = pinch_start_zoom() * (distance / start_distance);
                                        zoom.set(next.clamp(1.0, 4.0));
                                    }
                                }
                            } else {
                                pinch_start_distance.set(None);
                            }
                        });
                    },
                    onpointerup: move |evt| {
                        active_pointers.with_mut(|pointers| {
                            pointers.remove(&evt.data().pointer_id());
                        });
                        let remaining = active_pointers.read().len();
                        if remaining == 0 {
                            drag_start_pointer.set(None);
                            if !drag_moved() {
                                // click handler will run next
                            }
                        }
                        if remaining < 2 {
                            pinch_start_distance.set(None);
                        }
                        if remaining == 1 {
                            let pointers = active_pointers.read();
                            if let Some((_, &(x, y))) = pointers.iter().next() {
                                drag_start_pointer.set(Some((x, y)));
                                drag_start_pan.set((pan_x(), pan_y()));
                            }
                        }
                    },
                    onpointercancel: move |evt| {
                        active_pointers.with_mut(|pointers| {
                            pointers.remove(&evt.data().pointer_id());
                        });
                        pinch_start_distance.set(None);
                        drag_start_pointer.set(None);
                    },
                    onclick: move |evt: MouseEvent| {
                        if drag_moved() {
                            drag_moved.set(false);
                            return;
                        }
                        let mut next_selected: Option<LocationId> = None;

                        if let Some(web_event) = evt.data().as_ref().try_as_web_event()
                            && let Some(target) = web_event.target()
                            && let Ok(element) = target.dyn_into::<web_sys::Element>()
                            && let Some(hit) = element.closest("[data-code]").ok().flatten()
                            && let Some(code) = hit.get_attribute("data-code")
                            && let Ok(raw) = code.parse::<u32>()
                        {
                            let id = LocationId(raw);
                            next_selected = if selected() == Some(id) { None } else { Some(id) };
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
        "{} {{ fill: transparent !important; stroke: rgba(0, 245, 255, 0.42) !important; stroke-width: 0.85px !important; cursor: pointer; transition: fill 0.15s ease, filter 0.15s ease, stroke 0.15s ease; }}\n",
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
