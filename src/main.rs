use leptos::prelude::*;

#[component]
fn ProgressBar(
    progress: ReadSignal<i32>,
    #[prop(optional)]
    max: u16
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button 
            on:click=move |_| *set_count.write() += 1
            class:red=move || count.get() % 2 == 0
            class=(["font-bold", "rounded"], move || count.get() % 2 == 0)
        >
            "Click me:" {count}
        </button>
        <ProgressBar progress=count />        
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}