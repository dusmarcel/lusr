use leptos::prelude::*;

use crate::defaults;

// Bedarfsgemeinschaft

#[component]
pub fn Community(
    p: Memo<Option<u32>>,
    set_p: SignalSetter<Option<u32>>
) -> impl IntoView {
    let change_persons = move |e| set_p.set(Some(event_target_value(&e).parse::<u32>().unwrap_or(defaults::PERSONS)));

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Bedarfsgemeinschaft"
            </h2>
            <p>
                "Anzahl Personen: "
                <input type="number" min="1" value=move || p.get().unwrap_or(defaults::PERSONS) class="border-2 border-stone-400 rounded-lg px-1" on:change=change_persons />
            </p>
        </div>
    }
}