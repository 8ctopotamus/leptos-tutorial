use leptos::prelude::*;

/// Shows progress toward a goal
#[component]
fn ProgressBar(
    /// How much progress should be displayed
    #[prop(into)]
    progress: Signal<i32>,
    /// The maximum value of the progress bar
    #[prop(default = 100)]
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
    let double_count = move || count.get() * 2;

    view! {
        <button on:click=move |_| *set_count.write() += 1>
            "Click me:" {count}
        </button>
        <ProgressBar progress=count />        
        <ProgressBar progress=Signal::derive(double_count) />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}