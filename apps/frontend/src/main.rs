use dioxus::prelude::*;

const BACKEND_URL: &str = "http://localhost:3000";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut health_status = use_signal(|| "未確認".to_string());

    rsx! {
        h1 { "ケネクト" }
        p { "県をつないでいくゲームです" }

        button {
            onclick: move |_| async move {
                let url = format!("{}/health", BACKEND_URL);
                match reqwest::get(&url).await {
                    Ok(res) => {
                        let text = res.text().await.unwrap_or_else(|_| "エラー".to_string());
                        health_status.set(text);
                    }
                    Err(_) => {
                        health_status.set("接続失敗".to_string());
                    }
                }
            },
            "ヘルスチェック"
        }

        p { "バックエンド: {health_status}" }
    }
}
