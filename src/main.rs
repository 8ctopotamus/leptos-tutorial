use leptos::prelude::*;
use leptos_router::components::{Router, Route, Routes};
use leptos_router::path;

#[component]
fn Home() -> impl IntoView {
    view! {
        <h1>Home</h1>
    }
}

#[component]
fn Users() -> impl IntoView {
    view! {
        <h1>Users</h1>
    }
}

#[component]
fn UserProfile() -> impl IntoView {
    view! {
        <h1>User Profile</h1>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <nav>
                <a href="/">Home</a>
                <a href="/users">Users</a>
            </nav>
            <main>
                <Routes fallback=|| "Not found fallback.">
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/users") view=Users />
                    <Route path=path!("/users/:id") view=UserProfile />
                    <Route path=path!("/*any") view=|| view! { <h1>Not found</h1> } />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
