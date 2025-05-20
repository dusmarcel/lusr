use leptos::prelude::*;

use crate::{
    utils::format_euro,
    defaults
};

// Bedarfsgemeinschaft

#[component]
pub fn Community(
    e: Memo<Option<u32>>,
    set_e: SignalSetter<Option<u32>>,
    u25: Memo<Option<u32>>,
    set_u25: SignalSetter<Option<u32>>,
    u18: Memo<Option<u32>>,
    set_u18: SignalSetter<Option<u32>>,
    u14: Memo<Option<u32>>,
    set_u14: SignalSetter<Option<u32>>,
    u6: Memo<Option<u32>>,
    set_u6: SignalSetter<Option<u32>>,
    m: Memo<Option<f64>>,
    set_m: SignalSetter<Option<f64>>,
    kv: Memo<Option<f64>>,
    set_kv: SignalSetter<Option<f64>>,
    ku: Memo<Option<f64>>,
    set_ku: SignalSetter<Option<f64>>
) -> impl IntoView {
    let change_erwachsene = move |e| set_e.set(Some(event_target_value(&e).parse::<u32>().unwrap_or(defaults::ERWACHSENE)));
    let change_u25 = move |e| set_u25.set(Some(event_target_value(&e).parse::<u32>().unwrap_or(defaults::U25)));
    let change_u18 = move |e| set_u18.set(Some(event_target_value(&e).parse::<u32>().unwrap_or(defaults::U18)));
    let change_u14 = move |e| set_u14.set(Some(event_target_value(&e).parse::<u32>().unwrap_or(defaults::U14)));
    let change_u6 = move |e| set_u6.set(Some(event_target_value(&e).parse::<u32>().unwrap_or(defaults::U6)));
    let change_miete = move |e| set_m.set(Some(event_target_value(&e).parse::<f64>().unwrap_or(defaults::MIETE)));
    let change_kv = move |e| set_kv.set(Some(event_target_value(&e).parse::<f64>().unwrap_or(defaults::KV)));
    let change_ku = move |e| set_ku.set(Some(event_target_value(&e).parse::<f64>().unwrap_or(defaults::KU)));

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
                <select class="p-1 border-2 border-stone-400 rounded-lg" aria-label="Auswahl der Verfahrensart" id="adults" on:change=change_erwachsene>
                    <option value="1" selected=move || e.get().unwrap_or(defaults::ERWACHSENE) == 1>"Alleinstehende*r/alleinerziehende*r Erwachsene*r / Volljährige*r mit mindejähriger*m Partner*in"</option>
                    <option value="2" selected=move || e.get().unwrap_or(defaults::ERWACHSENE) == 2>"Zwei volljährige Partner*innen (verheiratet, unverheiratet oder Lebenspartner*innen)"</option>
                </select>
            </p>
            <p>
                <label for="u25">"Wie viele Erwachsene unter 25 Jahren, die im elterlichen Haushalt laben?"</label>
            </p>
            <p>
                <input type="number" id="u25" min="0" value=move || u25.get().unwrap_or(defaults::U25) class="px-1 border-2 border-stone-400 rounded-lg text-right" on:change=change_u25 />
            </p>
            <p>
                <label for="u18">"Wie viele Jugendliche von 14 bis 17 Jahren?"</label>
            </p>
            <p>
                <input type="number" id="u18" min="0" value=move || u18.get().unwrap_or(defaults::U18) class="px-1 border-2 border-stone-400 rounded-lg text-right" on:change=change_u18 />
            </p>
            <p>
                <label for="u14">"Wie viele Kinder von 6 bis 13 Jahren?"</label>
            </p>
            <p>
                <input type="number" id="u14" min="0" value=move || u14.get().unwrap_or(defaults::U14) class="px-1 border-2 border-stone-400 rounded-lg text-right" on:change=change_u14 />
            </p>
            <p>
                <label for="u6">"Wie viele Kinder von 0 bis 5 Jahren?"</label>
            </p>
            <p>
                <input type="number" id="u6" min="0" value=move || u6.get().unwrap_or(defaults::U6) class="px-1 border-2 border-stone-400 rounded-lg text-right" on:change=change_u6 />
            </p>
            <p>
                <label for="rent">"Miete inkl. Betriebskosten:"</label>
            </p>
            <p>
                <input type="text" id="rent" min="0.0" class="px-1 border-2 border-stone-400 rounded-lg text-right" value=move || format_euro(m.get().unwrap_or(defaults::MIETE)) on:change=change_miete prop:value=move || format_euro(m.get().unwrap_or(defaults::MIETE)) />
            </p>
            <p>
                <label for="hi">"Krankenversicherungskosten für private oder freiwillige gesetzliche Krankenversicherung. Soweit Personen im Rahmen ihrer Erwerbstätigkeit gesetzlich pflichtversichert sind, sind hier keine Kosten anzugeben:"</label>
            </p>
            <p>
                <input type="text" id="hi" min="0.0" class="px-1 border-2 border-stone-400 rounded-lg text-right" value=move || format_euro(kv.get().unwrap_or(defaults::KV)) on:change=change_kv prop:value=move || format_euro(kv.get().unwrap_or(defaults::KV)) />
            </p>
            <p>
                <label for="cm">"Unterhaltsverpflichtungen für nicht in der Bedarfsgemeinschaft lebende Personen:"</label>
            </p>
            <p>
                <input type="text" id="cm" min="0.0" class="px-1 border-2 border-stone-400 rounded-lg text-right" value=move || format_euro(ku.get().unwrap_or(defaults::KU)) on:change=change_ku prop:value=move || format_euro(ku.get().unwrap_or(defaults::KU)) />
            </p>
        </div>
    }
}