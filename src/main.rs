use leptos::prelude::*;
use reactive_stores::{Store, StoreFieldIterator};

#[derive(Store, Debug, Clone)]
pub struct DataStore {
    #[store(key: String = |row| row.key.clone())]
    rows: Vec<DatabaseEntry>,
}


#[derive(Store, Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32
}

#[component]
fn App() -> impl IntoView {
    let data = Store::new(DataStore { 
        rows: vec![
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
        ],
    });

    view! {
        <button on:click=move |_| {
            for row in data.rows().iter_unkeyed() {
                *row.value().write() *= 2;
            }
            leptos::logging::log!("{:?}", data.get());
        }>
            "Update values"
        </button>
        <For
            each=move || data.rows()
            key=|row| row.read().key.clone()
            children=move |child| {
                let value = child.value();
                view! {
                    <p>{move || value.get()}</p>
                }
            }
        />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}