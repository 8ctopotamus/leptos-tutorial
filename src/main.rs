use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (value, set_value) = signal(0);
    let is_odd = move || value.get() % 2 != 0;

    view! {
        <p>
            {move || if is_odd() {
                "odd"
            } else {
                "even"
            }}
        </p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
