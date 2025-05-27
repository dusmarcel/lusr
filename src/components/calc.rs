use leptos::prelude::*;
use reactive_stores::Store;

use crate::{
    defaults::ERWACHSENE, incomes::{adults::ErwachsenEinkommen, children::KindEinkommen}, utils::{absetzbetrag1, absetzbetrag2, absetzbetrag3, anr_einkommen, format_euro, werbungskostenpauschale}
};

// Berechnung der Absetzbeträge

#[component]
pub fn Calculations(
    e: Memo<Option<u32>>,
    k: Memo<u32>,
    mj_kinder: Memo<u32>,
    f: Memo<Option<bool>>,
    erwachsene_einkommen: Memo<Vec<ErwachsenEinkommen>>,
    kinder_einkommen: Memo<Vec<KindEinkommen>>
) -> impl IntoView {
    let maximum = move || if mj_kinder.get() > 0 { "1.500" } else { "1.200" };
    let vjk = Memo::new(move |_| k.get() - mj_kinder.get());
    let data = Store::new(move || kinder_einkommen.get());

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
                        <td>{ move || format_euro(0.0 - absetzbetrag3(erwachsene_einkommen.get()[0].brutto, f.get().unwrap_or(true), mj_kinder.get() > 0)) }</td>
                        <td>{ move || format_euro(anr_einkommen(erwachsene_einkommen.get()[0].brutto, erwachsene_einkommen.get()[0].netto, f.get().unwrap_or(true), mj_kinder.get() > 0)) }</td>
                    </tr>
                    <Show 
                        when=move || { e.get().unwrap_or(ERWACHSENE) == 2 }
                    >
                        <tr class="text-right">
                            <td>{ move || format_euro(erwachsene_einkommen.get()[1].netto) }</td>
                            <td>{ move || format_euro(0.0 - werbungskostenpauschale(erwachsene_einkommen.get()[1].brutto)) }</td>
                            <td>{ move || format_euro(0.0 - absetzbetrag1(erwachsene_einkommen.get()[1].brutto, f.get().unwrap_or(true))) }</td>
                            <td>{ move || format_euro(0.0 - absetzbetrag2(erwachsene_einkommen.get()[1].brutto, f.get().unwrap_or(true))) }</td>
                            <td>{ move || format_euro(0.0 - absetzbetrag3(erwachsene_einkommen.get()[1].brutto, f.get().unwrap_or(true), mj_kinder.get() > 0)) }</td>
                            <td>{ move || format_euro(anr_einkommen(erwachsene_einkommen.get()[1].brutto, erwachsene_einkommen.get()[1].netto, f.get().unwrap_or(true), mj_kinder.get() > 0)) }</td>
                        </tr>
                    </Show>
                </tbody>
            </table>
            <Show
                when=move || { k.get() > 0 }
            >
                <h3 class="text-xl">"Einkommen der Kinder"</h3>
                <Show
                    when=move || { k.get() > mj_kinder.get() || mj_kinder.get() != 1 }
                >
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
                                    "./. 10 % des Bruttoeinkommens über 1.000,00 EUR und bis zu 1.500,00 EUR"
                                </th>
                                <th class="px-1 text-left">
                                    "Anrechenbares Einkommen"
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <For
                                each=move || (data.get())()
                                key=|kind| kind.id
                                let:kind
                            >
                                <Show
                                    when=move || { k.get() as usize > kind.id && !(k.get() as usize > kind.id + 1 && mj_kinder.get() == 1) }
                                >
                                    <tr class="text-right">
                                        <td>{ move || format_euro(kinder_einkommen.get()[kind.id].netto) }</td>
                                        <td>{ move || format_euro(0.0 - werbungskostenpauschale(kinder_einkommen.get()[kind.id].brutto)) }</td>
                                        <td>{ move || format_euro(0.0 - absetzbetrag1(kinder_einkommen.get()[kind.id].brutto, f.get().unwrap_or(true))) }</td>
                                        <td>{ move || format_euro(0.0 - absetzbetrag2(kinder_einkommen.get()[kind.id].brutto, f.get().unwrap_or(true))) }</td>
                                        <td>{ move || format_euro(0.0 - absetzbetrag3(kinder_einkommen.get()[kind.id].brutto, f.get().unwrap_or(true), true)) }</td>
                                        <td>{ move || format_euro(anr_einkommen(kinder_einkommen.get()[kind.id].brutto, kinder_einkommen.get()[kind.id].netto, f.get().unwrap_or(true), true)) }</td>
                                    </tr>
                                </Show>                               
                            </For>
                        </tbody>
                    </table>
                </Show>
                <Show
                    when=move || { mj_kinder.get() == 1 }
                >
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
                                    "./. 10 % des Bruttoeinkommens über 1.000,00 EUR und bis zu 1.200,00 EUR"
                                </th>
                                <th class="px-1 text-left">
                                    "Anrechenbares Einkommen"
                                </th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="text-right">
                                <td>{ move || format_euro(kinder_einkommen.get()[vjk.get() as usize].netto) }</td>
                                <td>{ move || format_euro(0.0 - werbungskostenpauschale(kinder_einkommen.get()[vjk.get() as usize].brutto)) }</td>
                                <td>{ move || format_euro(0.0 - absetzbetrag1(kinder_einkommen.get()[vjk.get() as usize].brutto, f.get().unwrap_or(true))) }</td>
                                <td>{ move || format_euro(0.0 - absetzbetrag2(kinder_einkommen.get()[vjk.get() as usize].brutto, f.get().unwrap_or(true))) }</td>
                                <td>{ move || format_euro(0.0 - absetzbetrag3(kinder_einkommen.get()[vjk.get() as usize].brutto, f.get().unwrap_or(true), false)) }</td>
                                <td>{ move || format_euro(anr_einkommen(kinder_einkommen.get()[vjk.get() as usize].brutto, kinder_einkommen.get()[vjk.get() as usize].netto, f.get().unwrap_or(true), false)) }</td>
                            </tr>
                        </tbody>
                    </table>
                </Show>                
            </Show>
        </div>
    }
}