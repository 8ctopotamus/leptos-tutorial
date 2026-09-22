use leptos::prelude::*;
use gloo_timers::future::TimeoutFuture;

async fn load_data(value: i32) -> i32 {
    TimeoutFuture::new(2_000).await;
    value * 10
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    // a resource will only load once if it doesn't read any reactive data
    let stable = LocalResource::new(|| load_data(1));

    // tracks count and calls load_data whenever it changes
    let async_data = LocalResource::new(move || load_data(count.get()));

    // we can access the resource values with .get()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            .unwrap_or_else(|| "Loading".into())
    };

    view! {
        <button
            on:click=move |_| *set_count.write() += 1
        >
            "Click me"
        </button>
        <p>
            <code>"stable"</code>": " {move || stable.get()}
        </p>
        <p>
            <code>"count"</code>": " {count}
        </p>
        <p>
            <code>"async_value"</code>": "
            {async_result}
            <br/>
        </p>

    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
