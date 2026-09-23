use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, _set_count) = signal::<u32>(0);

    provide_context(count);

    view! {
        <FancyMath />
    }
}

#[component]
fn FancyMath() -> impl IntoView {
    let count = use_context::<ReadSignal<u32>>()
        .expect("there to be a `count` signal provided");
    let is_even = move || count.get() & 1 == 0;

    view! {
        <div>
            "The number " 
            <strong>{count}</strong>
            {move || if is_even() {
                " is"
            } else {
                " is not"
            }}
            " even."
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
