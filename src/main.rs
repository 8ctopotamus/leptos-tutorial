use leptos::prelude::*;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>
}

#[component]
fn App() -> impl IntoView {
    let (data, _set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: RwSignal::new(10),
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: RwSignal::new(20),
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: RwSignal::new(15),
        },
    ]);

    view! {
        <button on:click=move |_| {
            for row in &*data.read() {
                row.value.update(|value| *value *= 2);
            }
            leptos::logging::log!("{:?}", data.get());
        }>
            "Update values"
        </button>
        <For
            each=move || data.get()
            key=|state| state.key.clone()
            let(child)
        >
           <p>{child.value}</p>        
        </For>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}