use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
        
    let double_count = move || count.get() * 2;

    view! {
        <button 
            on:click=move |_| *set_count.write() += 1
            class:red=move || count.get() % 2 == 0
            class=(["font-bold", "rounded"], move || count.get() % 2 == 0)
        >
            "Click me:" {count}
        </button>
        <p>
            "Frozen count: " 
            {count.get()}
        </p>
        <p>
            "Double count: " 
            {double_count}
        </p>
        <progress
            max="50"
            value=count 
        />
        <br />
        <button
            on:click=move |_| *set_count.write() += 10 
            style="position: relative;"
            style:left=move || format!("{}px", count.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            style=("--columns", move || count.get().to_string())
        >
            "Click to move"
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}