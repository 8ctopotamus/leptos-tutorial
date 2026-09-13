use leptos::prelude::*;

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = signal(Ok(0));

    view! {
        <h1>"Error Handling"</h1>
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
        <ErrorBoundary fallback=|errors| view! {
            <div class="error">
                <p>"Not a number! Errors: "</p>
                <ul>
                    {move || errors.get()
                        .into_iter()
                        .map(|(_, e)| view! { <li>{e.to_string()}</li> } ) 
                        .collect::<Vec<_>>()
                    }
                </ul>
            </div>
        }>
            <p>"You entered: " <strong>{value}</strong></p>
        </ErrorBoundary>
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
