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
    
    // shorter Option syntax
    let message_2 = move || is_odd().then(|| "Bing bing bing");

    // match statement
    let match_message = move || {
        match value.get() {
            0 => "Zero",
            1 => "One",
            n if is_odd() => "Odd",
            _ => "Even"
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
        <p>"Using Option shorter syntax: " {message_2}</p>
        <p>"Using match: " {match_message}</p>
        // Show only re-renders when the condition result has changed
        <Show
            when=move || { value.get() > 5 }
            fallback=|| view! { "Some <Small /> component"  }
        >
            "Some <Big /> component"
        </Show>

    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
