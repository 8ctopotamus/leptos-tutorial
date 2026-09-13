use leptos::prelude::*;

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = signal(Ok(0));

    view! {
        <label>
            "Type an integer (or not!)"
            <input 
                on:input:target=move |ev| {
                    // true to parse value into a i32
                    set_value.set(ev.target().value().parse::<i32>());
                } 
                type="number"
            />
        </label>
        // value will show up blank in the DOM if an error is thrown. 
        // (try typing numbers vs strings)
        <p>"You entered: " <strong>{value}</strong></p>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {  
        <NumericInput />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
