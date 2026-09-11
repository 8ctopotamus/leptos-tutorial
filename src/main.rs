use leptos::prelude::*;
use leptos::html;
use leptos::ev::SubmitEvent;

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal("Uncontrolled".to_string());

    let input_element: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        // extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit>
            <input
                value=name
                node_ref=input_element
                type="text"
            />
            <input type="submit" value="Submit" />
        </form>       
        <p>"Name is: " {name}</p>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}