use leptos::prelude::*;
use leptos::logging::{log};

#[component]
fn App() -> impl IntoView {
    let (count, _set_count) = signal(0);

    let _double_count = move || count.get() * 2;
    let count_is_odd = move || count.get() & 1 == 1;
    let text = move || if count_is_odd() {
        "odd"
    } else {
        "even"
    };

    // effects automatically track the signals it depends
    // and re-reuns when they change
    Effect::new(move |_| {
        log!("text = {}", text());
    });

    view! {
        <p>{move || text().to_uppercase()}</p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
