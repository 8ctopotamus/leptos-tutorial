use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (move, set_move) = signal(10)
    
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
            {move || count.get() * 2}
        </p>
        <button
            style="position: absolute"
            style:left=move || format("{}px", count.get() + 100)
        >
            "Click to move"
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> })
}