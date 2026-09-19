use leptos::prelude::*;


#[component]
fn App() -> impl IntoView {
    // making signals depend on each other - good options:

    // 1) B is a function of A. Create a signal for A and a derived signal or memo for B.
    // A
    let (count, _set_count) = signal(1);
    // B is a function of A
    let _derived_signal_double_count = move || count.get() * 2;
    // B is a function of A
    let _memoized_double_count = Memo::new(move |_| count.get() * 2);

    // 2) C is a function of A and some other thing B. Create signals for A and B and a derived signal or memo for C
    // A
    let (first_name, _set_first_name) = signal("Bridget".to_string());
    // B
    let (last_name, _set_last_name) = signal("Jones".to_string());
    // C is a function of A and B
    let _full_name = move || format!("{} {}", &*first_name.read(), &*last_name.read());

    // 3) A and B are independent signals, but sometimes updated at the same time. When you make the call to update A, make a separate call to update B.
    // A
    let (age, set_age) = signal(32);
    // B
    let (favorite_number, set_favorite_number) = signal(42);
    // use this to handle a click on a `Clear` button    
    let clear_handler = move |_| {
        // update both A and B
        set_age.set(0);
        set_favorite_number.set(0);
    };

    view!  { 
        <p>{move || age.get()}</p>
        <p>{move || favorite_number.get()}</p>
        <button on:click=clear_handler>
            "Clear"
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
