use leptos::prelude::*;

use crate::{
    incomes::{adults::ErwachsenEinkommen, children::KindEinkommen},
    utils::{absetzbetrag1, absetzbetrag2, absetzbetrag3, anr_einkommen, format_euro, werbungskostenpauschale}
};

// Berechnung der Absetzbeträge

#[component]
pub fn Calculations(
    f: Memo<Option<bool>>,
    mj_kinder: Memo<bool>,
    erwachsene_einkommen: Memo<Vec<ErwachsenEinkommen>>,
    kinder_einkommen: Memo<Vec<KindEinkommen>>
) -> impl IntoView {
    let maximum = move || if mj_kinder.get() { "1.500" } else { "1.200" };

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Berechnung der Absetzbeträge"
            </h2>
            <table>
                <thead>
                    <tr>
                        <th class="px-1 text-left">
                            "Einkommen netto"
                        </th>
                        <th class="px-1 text-left">
                            "./. Werbungskostenpauschale"
                        </th>
                        <th class="px-1 text-left">
                            "./. 20 % des Bruttoeinkommens über 100,00 EUR und bis zu 520,00 EUR"
                        </th>
                        <th class="px-1 text-left">
                            "./. 30 % des Bruttoeinkommens über 520,00 EUR und bis zu 1.000,00 EUR"
                        </th>
                        <th class="px-1 text-left">
                            "./. 10 % des Bruttoeinkommens über 1.000,00 EUR und bis zu "{maximum}" EUR"
                        </th>
                        <th class="px-1 text-left">
                            "Anrechenbares Einkommen"
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr class="text-right">
                        <td>{ move || format_euro(erwachsene_einkommen.get()[0].netto) }</td>
                        <td>{ move || format_euro(0.0 - werbungskostenpauschale(erwachsene_einkommen.get()[0].brutto)) }</td>
                        <td>{ move || format_euro(0.0 - absetzbetrag1(erwachsene_einkommen.get()[0].brutto, f.get().unwrap_or(true))) }</td>
                        <td>{ move || format_euro(0.0 - absetzbetrag2(erwachsene_einkommen.get()[0].brutto, f.get().unwrap_or(true))) }</td>
                        <td>{ move || format_euro(0.0 - absetzbetrag3(erwachsene_einkommen.get()[0].brutto, f.get().unwrap_or(true), mj_kinder.get())) }</td>
                        <td>{ move || format_euro(anr_einkommen(erwachsene_einkommen.get()[0].brutto, erwachsene_einkommen.get()[0].netto, f.get().unwrap_or(true), mj_kinder.get())) }</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}