use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let email = RwSignal::new("".to_string());
    let favorite_color = RwSignal::new("red".to_string());
    let spam_me = RwSignal::new(true);

    view! {
        <input 
            bind:value=(name, set_name)
            type="text"
        />
        <input
            bind:value=email
            type="email"
        />
        <label>
            "Please send me lots of spam email."
            <input 
                bind:checked=spam_me
                type="checkbox" 
            />
        </label>
        <fieldset>
            <legend>"Favorite color"</legend>
            <label>
                "Red"
                <input 
                    name="color"
                    value="red"
                    bind:group=favorite_color
                    type="radio"
                />
            </label>
            <label>
                "Green"
                <input 
                    name="color"
                    value="green"
                    bind:group=favorite_color
                    type="radio"
                />
            </label>
            <label>
                "Blue"
                <input 
                    name="color"
                    value="blue"
                    bind:group=favorite_color
                    type="radio"
                />
            </label>
        </fieldset>

        <p>"Your favorite color is " {favorite_color} "."</p>
        <p>"Name is: " {name}</p>
        <p>"Email is: " {email}</p>
        <Show when=move || spam_me.get()>
            <p>"You'll receive cool bonus content!"</p>
        </Show>        
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}