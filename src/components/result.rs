use std::default;

use leptos::prelude::*;
use leptos_use::{use_intl_number_format, CurrencyDisplay, NumberStyle, UseIntlNumberFormatOptions};

use crate::utils::format_euro;

// Ergebnis

#[component]
pub fn Result(
    sb: Memo<f64>,
    ae: ReadSignal<f64>
) -> impl IntoView {
    let ratio = Memo::new(move |_| ae.get() / sb.get());

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Ergebnis"
            </h2>
            <p>
                "Dem Bedarf in Höhe von "
                { move || format_euro(sb.get()) }
                " steht ein Einkommen in Höhe von "
                { move || format_euro(ae.get()) }
                " gegenüber. "
                { move || if sb.get() == ae.get() {
                    "Damit entspricht das Einkommen genau dem Bedarf"
                } else {
                    "Damit übersteigt "
                } }
                { move || if sb.get() > ae.get() {
                   Some("der Bedarf das Einkommen um ")
                } else { None } }
                { move || if sb.get() > ae.get() {
                   Some(format_euro(sb.get() - ae.get()))
                } else { None } }
                { move || if ae.get() > sb.get() {
                   Some("das Einkommen den Bedarf um ")
                } else { None } }
                { move || if ae.get() > sb.get() {
                   Some(format_euro(ae.get() - sb.get()))
                } else { None } }
                "."
            </p>
            <p>
                "Der Lebensunterhalt ist also zu "
                <span class={ move || if ratio.get() > 1.0 { "font-semibold text-green-500" } else { "font-semibold" } }>
                { move || use_intl_number_format(
                    UseIntlNumberFormatOptions::default()
                        .locale("de-DE")
                        .style(NumberStyle::Percent)
                        .maximum_fraction_digits(2)
                )
                .format::<f64>(ratio)
                .get()
                }
                </span>
                " und damit "
                <span class={ move || if ratio.get() > 1.0 { "font-semibold text-green-500" } else { "font-semibold text-red-500 underline" } }>
                { move || if ratio.get() > 1.0 { "vollständig" } else { "nicht vollständig" } }
                </span>
                " gesichert."
                { move || if ratio.get() > 0.5 { Some("Der Lebensunterhalt ist allerdings ") } else { None } }
                <span class={ move || if ratio.get() > 0.75 { "text-yellow-300" } else if ratio.get() > 0.5 { "text-orange-500" } else { ""} }>
                "überwiegend gesichert."
                </span>
            </p>
        </div>
    }
}