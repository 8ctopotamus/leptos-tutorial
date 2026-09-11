use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (some_textarea_value, set_some_textarea_value) = signal("".to_string());

    let (select_value, set_select_value) = signal(0i32);
    
    view! {
        <textarea
            prop:value=move || some_textarea_value.get()
            on:input:target=move |e| set_some_textarea_value.set(e.target().value())
        >
            {some_textarea_value}
        </textarea>
        <p><strong>"Message: "</strong> {some_textarea_value}</p>
        <select
            prop:value=select_value
            on:change:target=move |e| {
                set_select_value.set(e.target().value().parse().unwrap());
            } 
        >
            <option value="0">"0"</option>
            <option value="1">"1"</option>
            <option value="2">"2"</option>
        </select>
        <button on:click=move |_| set_select_value.update(|n| {
            if *n == 2 {
                *n = 0;
            } else {
                *n += 1;
            }
        })>
            "Next option"
        </button>
        <p><strong>"Select value: "</strong> {select_value}</p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}