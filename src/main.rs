use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (names, set_names) = signal(Vec::new());

    // this code is inefficient because we are cloning the whole Vec<string>
    // just to check .is_empty(), and then throwing away the clone
    // let _ = if names.get().is_empty() {
        // this .set is also inefficient because we are replacing the value with a whole new Vec<String>...
        // set_names.set(vec!["Alice".to_string()]);
    // };

    // instead we can:

    // use names by reference to avoid the clone using `.read()`
    if names.read().is_empty() {
        // mutate the original Vec<String> in place using `.write()`
        set_names.write().push("Alice".to_string());
    }

    view!  {
        <ul>
            {names.get().into_iter()
                .map(|n| view! { <li>{n}</li> })
                .collect::<Vec<_>>()}
        </ul>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
