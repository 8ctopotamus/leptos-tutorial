use leptos::prelude::*;
use reactive_stores::Store;

#[derive(Clone, Debug, Default, Store)]
struct GlobalState {
    count: i32,
    name: String,
}

#[component]
fn App() -> impl IntoView {
    provide_context(Store::new(GlobalState::default()));

    view! {
        <GlobalStateCounter />
    }
}

#[component]
fn GlobalStateCounter() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();

    // this gives us reactive caccess to the `count` field only
    let count = state.count();

    view! {
        <div>
            <button
                on:click=move |_| {
                    *count.write() += 1;
                }
            >
                "Increment Global Count"
            </button>
            <br />
            <span>"Count is: " {move || count.get()}</span>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
