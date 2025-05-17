use leptos::prelude::*;

use crate::defaults;

// Einkommen

#[component]
pub fn Income(
    a: Memo<Option<u32>>,
    c: Memo<u32>,
) -> impl IntoView {

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Einkommen"
            </h2>
            <p>
                "Dem ermittelten Bedarf müssen wir jetzt das Einkommen der Bedarfsgemeinschaft gegenüberstellen."
            </p>
            <p>
                "Die Bedarfsgemeinschaft besteht aus "
                { move || a.get().unwrap_or(defaults::ADULTS) }
                " Erwachsennen und "
                { move || c.get() }
                " Kindern."
            </p>
        </div>
    }
}