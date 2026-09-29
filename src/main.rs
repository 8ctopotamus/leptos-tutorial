use leptos::prelude::*;
use leptos_router::components::{Router, Route, Routes, ParentRoute, Outlet, A};
use leptos_router::hooks::use_params_map;
use leptos_router::path;

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <h1>Contact App</h1>
            <nav>
                <a href="/">"Home"</a>
                <a href="/contacts">"Contacts"</a>
            </nav>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=|| view! {  
                        <h3>"Home"</h3>
                    } />
                    <ParentRoute 
                        path=path!("/contacts") 
                        view=ContactList
                    >
                        // if no id specified, fall back
                        <ParentRoute path=path!(":id") view=ContactInfo>
                            <Route path=path!("") view=|| view! {
                                <div class="tab">
                                    "(Contact Info)"
                                </div>
                            } />
                            <Route path=path!("conversations") view=|| view! {
                                <div class="tab">
                                    "(Conversations)"
                                </div>
                            } />
                        </ParentRoute>
                        // if no id specified, fall back
                        <Route path=path!("") view=|| view! {
                            <div class="select-user">
                                "Select a user to view contact info."
                            </div>
                        } />                        
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn ContactList() -> impl IntoView {
    view! {
        <h3>"Contacts"</h3>
        <div class="contact-list-contacts">
            <A href="alice">"Alice"</A>
            <A href="bob">"Bob"</A>
            <A href="steve">"Steve"</A>
        </div>
        <Outlet/>
    }
}

#[component]
fn ContactInfo() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id").unwrap_or_default();

    let name = move || match id().as_str() {
        "alice" => "Alice",
        "bob" => "Bob",
        "steve" => "Steve",
        _ => "User not found",
    };

    view! {
        <h4>{name}</h4>
        <div class="contact-info">
            <A href="" exact=true>"Contact Info"</A>
            <A href="conversations">"Conversations"</A>
        </div>
        <Outlet />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
