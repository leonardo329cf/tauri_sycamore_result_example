use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::rt::Event;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // the catch lets you handle the js error transforming it into a Result<JsValue, JsValue>
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str
}

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let name = create_signal(cx, String::new());
    let greet_msg = create_signal(cx, String::new());

    let greet = move |e: Event| {
        e.prevent_default();
        spawn_local_scoped(cx, async move {
            let new_msg =
                invoke("greet", to_value(&GreetArgs { name: &name.get() }).unwrap()).await;
            match new_msg {
                Ok(ok_msg) => greet_msg.set(serde_wasm_bindgen::from_value::<String>(ok_msg).unwrap()),
                Err(err_msg)  => greet_msg.set(serde_wasm_bindgen::from_value::<String>(err_msg).unwrap()),
            }
            
        })
    };

    view! { cx,
        main(class="container") {
            div(class="row") {
                a(href="https://tauri.app",target="_blank") {
                    img(src="public/tauri.svg",class="logo tauri",alt="Tauri logo")
                }
                a(href="https://sycamore-rs.netlify.app",target="_blank") {
                    img(src="public/sycamore.svg",class="logo sycamore",alt="Sycamore logo")
                }
            }
            p {
                "Hi this shows how to handle errors comming from you tauri commands"
            }
            p {
                "This Greet only greets Leo, else will return an Err telling you to get out"
            }
            form(class="row",on:submit=greet) {
                input(id="greet-input",bind:value=name,placeholder="Enter a name...")
                button(type="submit") {
                    "Greet"
                }
            }
            p {
                b {
                    (greet_msg.get())
                }
            }
        }
    }
}
