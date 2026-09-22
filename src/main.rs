use leptos::prelude::*;
use gloo_timers::future::TimeoutFuture;

async fn important_api_call(name: String) -> String {
    TimeoutFuture::new(2_000).await;
    name.to_ascii_uppercase()
}

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal("Josh".to_string());

    let async_data = LocalResource::new(move || important_api_call(name.get()));

    view! {
        <input 
            on:change:target=move |ev| set_name.set(ev.target().value())
            prop:value=name 
        />
        <p><code>"name:"</code> {name}</p>
        <Suspense
            // the fallback will show whenever a resource
            // read "under" the suspense is loading
            fallback=move || view! { <p>"Loading..."</p> }
        >
            <p>
                "Your shouting name is "
                {move || Suspend::new(async move {
                    async_data.await
                })}
            </p>
        </Suspense>
        <Suspense
            // the fallback will show whenever a resource
            // read "under" the suspense is loading
            fallback=move || view! { <p>"Loading..."</p> }
        >
            <p>
                "Which should be the same as... "
                {move || async_data.get().as_deref().map(ToString::to_string)}
            </p>
        </Suspense>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
