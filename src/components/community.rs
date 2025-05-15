use leptos::prelude::*;

use crate::defaults;

// Bedarfsgemeinschaft

#[component]
pub fn Community(
    a: Memo<Option<u32>>,
    set_a: SignalSetter<Option<u32>>
) -> impl IntoView {
    let change_adults = move |e| set_a.set(Some(event_target_value(&e).parse::<u32>().unwrap_or(defaults::ADULTS)));

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Bedarfsgemeinschaft"
            </h2>
            <p>
                "Im ersten Schritt müssen wir die Regelbedarfe ermitteln. Dazu müssen wir zunächst wissen, wie viele Personen in der Bedarfsgemeinschaft leben."
            </p>
            <p>
                <label for="adults">"Anzahl erwachsene Personen:"</label>
            </p>
            <p>
                <select class="p-1 border-2 border-stone-400 rounded-lg" aria-label="Auswahl der Verfahrensart" id="adults" on:change=change_adults>
                    <option value="1" selected=move || a.get().unwrap_or(defaults::ADULTS) == 1>"Alleinstehende*r/alleinerziehende*r Erwachsene*r / Volljährige*r mit mindejähriger*m Partner*in"</option>
                    <option value="2" selected=move || a.get().unwrap_or(defaults::ADULTS) == 2>"Zwei volljährige Partner*innen (verheiratet, unverheiratet oder Lebenspartner*innen)"</option>
                </select>
            </p>
        </div>
    }
}