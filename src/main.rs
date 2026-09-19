use leptos::prelude::*;
use leptos::html::p;

#[component]
fn App() -> impl IntoView {
    let (value, _set_value) = signal(0);

    // use component props builder
    // Show(
    //     ShowProps::builder()
    //         .when(move || value.get() > 5)
    //         .fallback(|| p().child("I will appear if `value` is 5 or lower"))
    //         .children(ToChildren::to_children(|| {
    //             p().child("I will appear if `value is above 5")
    //         }))
    //         .build(),
    // )

    // or directly build the props struct
    Show(ShowProps {
        when: move || value.get() > 5,
        fallback: (|| p().child("I will appear if `value` is 5 or lower")).into(),
        children: ToChildren::to_children(|| p().child("I will appear if `value is above 5")),
    })
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
