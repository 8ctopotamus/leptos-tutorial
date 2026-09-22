use leptos::prelude::*;
use gloo_timers::future::TimeoutFuture;

async fn important_api_call(id: usize) -> String {
    TimeoutFuture::new(2_000).await;
    match id {
        0 => "Alice",
        1 => "Bob",
        2 => "Carol",
        _ => "User not found"
    }.to_string()
}

#[component]
fn App() -> impl IntoView {
    let (tab, set_tab) = signal(0);
    let (pending, set_pending) = signal(false);

    // this will reload every time `tab` changes
    let user_data = LocalResource::new(move || important_api_call(tab.get()));
    
    view! {
        <div>
            <button
                on:click=move |_| set_tab.set(0)
                class:selected=move || tab.get() == 0 
            >
                "Tab A"
            </button>
            <button
                on:click=move |_| set_tab.set(1)
                class:selected=move || tab.get() == 1 
            >
                "Tab B"
            </button>
            <button
                on:click=move |_| set_tab.set(2)
                class:selected=move || tab.get() == 2 
            >
                "Tab C"
            </button>
        </div>
        <p>
            {move || if pending.get() {
                "Hang on..."
            } else {
                "Ready."
            }}
        </p>
        <Transition
            fallback=move || view! { <p>"Loading initial data..."</p> }
            // the fallback will show initially
            // on subsequent reloads, the current child will
            // continue showing
            set_pending
        >
            <p>
                {move || user_data.read().as_deref().map(ToString::to_string)}
            </p>
        </Transition>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
