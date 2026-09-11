use leptos::prelude::*;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32
}

#[component]
fn App() -> impl IntoView {
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 15,
        },
    ]);

    view! {
        <button on:click=move |_| {
            set_data.update(|items| {
                for item in items.iter_mut() {
                    item.value *= 2;
                }
            });
        }>
            "Update values"
        </button>
        <ForEnumerate
            each=move || data.get()
            key=|state| state.key.clone()
            children=move |index, _| {
                let value = Memo::new(move |_| {
                    data.with(|data| data.get(index.get()).map(|d| d.value).unwrap_or(0))
                });
                view! {
                    <p>{value}</p>
                }
            }
        />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}