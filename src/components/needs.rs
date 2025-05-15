use leptos::prelude::*;

use crate::defaults;

// Bedarfe

#[component]
pub fn Needs(
    a: Memo<Option<u32>>
) -> impl IntoView {

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Bedarfe"
            </h2>
            <p>
                "Anzahl Personen: " { move || a.get().unwrap_or(defaults::ADULTS) }
            </p>
        </div>
    }
}