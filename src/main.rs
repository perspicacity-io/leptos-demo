use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn main() {
    //unsafe { alert("test") };
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <div></div>
        <button
            on:click=move |_| {
                set_count.set(count.get() + 1)
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
    }
}
