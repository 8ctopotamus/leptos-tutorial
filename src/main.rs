use leptos::prelude::*;


#[component]
fn App() -> impl IntoView {
    let values = vec![0, 1, 2];

    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));
    let counter_buttons = counters.map(|count| {
        view! {
            <li>
                <button on:click=move |_| *count.write() += 1>
                    {count}
                </button>
            </li>
        }
    })
    .collect_view();
    
    view! {
        <p>{values.clone()}</p>
        <ul>
            {values.clone().into_iter()
                .map(|n| view! { <li>{n}</li> })
                .collect::<Vec<_>>()}
        </ul>
        // or using the .collect_view() helper
        <ul>
            {values.clone().into_iter()
                .map(|n| view! { <li>{n}</li> })
                .collect_view()}
        </ul>
        <ul>{counter_buttons}</ul>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}