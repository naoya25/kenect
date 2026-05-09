use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        h1 { "ケネクト" }
        p { "県をつないでいくゲームです" }
    }
}
