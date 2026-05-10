use dioxus::prelude::*;

#[component]
pub fn SetupScreen(on_start: EventHandler<usize>) -> Element {
    let mut player_count = use_signal(|| 2usize);

    rsx! {
        div {
            h1 { "ケネクト" }
            p { "県をつないでいくゲームです" }

            label { "プレイヤー数：" }
            input {
                r#type: "number",
                min: "1",
                max: "10",
                value: "{player_count}",
                oninput: move |e| {
                    if let Ok(n) = e.value().parse::<usize>() {
                        if n >= 1 { player_count.set(n); }
                    }
                }
            }

            button {
                onclick: move |_| on_start.call(player_count()),
                "ゲーム開始"
            }
        }
    }
}
