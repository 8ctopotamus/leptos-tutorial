use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());

    view! {
        <input 
            prop:value=name
            on:input:target=move |e| {
                set_name.set(e.target().value());
            }
            type="text"
        />
        <p>"Name is: " {name}</p> 
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}