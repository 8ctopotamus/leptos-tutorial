use leptos::prelude::*;

#[component]
pub fn TakesChildren<F, IV>(
    render_prop: F,
    children: Children
) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h1><code>"<TakesChildren />"</code></h1>
        <h2>"Render Prop:"</h2>
        {render_prop()}
        <hr />
        <h2>"Children"</h2>
        {children()}
    }
}

#[component]
fn App() -> impl IntoView {
    view! {  
        <TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
            "Some text"
            <span>"A span"</span>
        </TakesChildren>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
