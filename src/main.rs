use leptos::prelude::*;
use leptos_router::components::{Form, Route, Router, Routes};
use leptos_router::hooks::{use_params_map, use_query_map};
use leptos_router::path;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <h1><code>"<Form />"</code></h1>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("") view=FormExample />
                </Routes>
            </main>
            <footer>"Some footer info..."</footer>
        </Router>
    }
}

#[component]
fn FormExample() -> impl IntoView {
    let query = use_query_map();
    let name = move || query.read().get("name").unwrap_or_default();
    let number = move || query.read().get("number").unwrap_or_default();
    let select = move || query.read().get("select").unwrap_or_default();

    view! {
        <table>
            <tr>
                <td><code>"name"</code></td>
                <td>{name}</td>
            </tr>
            <tr>
                <td><code>"number"</code></td>
                <td>{number}</td>
            </tr>
            <tr>
                <td><code>"select"</code></td>
                <td>{select}</td>
            </tr>
        </table>
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="">
            <input type="text" name="name" value=name />
            <input type="number" name="number" value=number />
            <select name="select">
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
            <input type="submit" /> // submitting causes a client-side navigation, not a full reload.
        </Form>

        <h2>"Automatic Submission"</h2>
        <Form method="GET" action="">
            <input
                type="text"
                name="name"
                value=name 
                oninput="this.form.requestSubmit()"
            />
            <input
                type="number"
                name="number"
                value=number
                oninput="this.form.requestSubmit()"
            />
            <select 
                name="select" 
                onchange="this.form.requestSubmit()"
            >
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
            <input type="submit" /> // submitting causes a client-side navigation, not a full reload.
        </Form>

    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
