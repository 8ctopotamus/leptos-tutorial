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
pub fn WrapsChildren(children: ChildrenFragment) -> impl IntoView {
    // children() returns a `Fragment`, which has a `nodes` field that contains a Vec<View>
    // this means we can iterate over the children to create something new!
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect::<Vec<_>>();
    view! {
        <h1><code>"<WrapsChildren />"</code></h1>
        <ul>{children}</ul>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {  
        <TakesChildren render_prop=|| view! { <p>"Hi, there!"</p> }>
            "Some text"
            <span>"A span"</span>
        </TakesChildren>

        <WrapsChildren>
            "A"
            "B"
            "C"
            "D"
        </WrapsChildren>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
