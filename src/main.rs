use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (value, set_value) = signal(1);
    let is_odd = move || value.get() % 2 != 0;

    // using Option
    let message = move || {
        if is_odd() {
            Some("Ding ding ding")
        } else {
            None
        }
    };

    view! {
        <p>
           "inline if: " {move || if is_odd() {
                "odd"
            } else {
                "even"
            }}
        </p>    
        <p>"Using Option: " {message}</p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
