use leptos::prelude::*;
//use leptos::web_sys::console;

use crate::{
    utils::format_euro,
    defaults,
    incomes::{
        adults::{erwachsene_einkommen_to_string, ErwachsenEinkommen},
        children::{kinder_einkommen_to_string, KindEinkommen}
    }
};

// Einkommen

#[component]
pub fn Income(
    e: Memo<Option<u32>>,
    k: Memo<u32>,
    erwachsene_einkommen: Memo<Vec<ErwachsenEinkommen>>,
    set_ee: SignalSetter<Option<String>>,
    kinder_einkommen: Memo<Vec<KindEinkommen>>,
    set_ke: SignalSetter<Option<String>>
) -> impl IntoView {
    let couple = Memo::new(move |_| e.get().unwrap_or(defaults::ERWACHSENE) == 2);

    let change_ee0_brutto = move |e| {
        let mut new_adults_incomes = erwachsene_einkommen.get().clone();
        new_adults_incomes[0].brutto = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
        set_ee.set(Some(erwachsene_einkommen_to_string(&new_adults_incomes)));
    };
    let change_ee0_netto = move |e| {
        let mut new_adults_incomes = erwachsene_einkommen.get().clone();
        new_adults_incomes[0].netto = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
        set_ee.set(Some(erwachsene_einkommen_to_string(&new_adults_incomes)));
    };
    let change_ee1_brutto = move |e| {
        let mut new_adults_incomes = erwachsene_einkommen.get().clone();
        new_adults_incomes[1].brutto = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
        set_ee.set(Some(erwachsene_einkommen_to_string(&new_adults_incomes)));
    };
    let change_ee1_netto = move |e| {
        let mut new_adults_incomes = erwachsene_einkommen.get().clone();
        new_adults_incomes[1].netto = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
        set_ee.set(Some(erwachsene_einkommen_to_string(&new_adults_incomes)));
    };
    let change_ke = move |e| set_ke.set(Some(kinder_einkommen_to_string(&kinder_einkommen.get())));

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Einkommen"
            </h2>
            <p>
                "Dem ermittelten Bedarf müssen wir jetzt das Einkommen der Bedarfsgemeinschaft gegenüberstellen."
            </p>
            <table>
                <tr>
                    <td class="px-1">
                        "Einkommen brutto:"
                    </td>
                    <td class="px-1">
                        <input type="text" min="0.0" class="px-1 border-2 border-stone-400 rounded-lg text-right" value={ move || format_euro(erwachsene_einkommen.get()[0].brutto) } prop:value={ move || format_euro(erwachsene_einkommen.get()[0].brutto) } on:change=change_ee0_brutto />
                    </td>
                    <td class={ move || if couple.get() { "visible" } else { "hidden" }}>
                        <input
                            type="text"
                            min="0.0"
                            class="px-1 border-2 border-stone-400 rounded-lg text-right"
                            value={
                                move || if let Some(ee) = erwachsene_einkommen.get().get(1) {
                                    format_euro(ee.brutto)
                                } else {
                                    format_euro(0.0)
                                } 
                            } 
                            prop:value={
                                move || if let Some(ee) = erwachsene_einkommen.get().get(1) {
                                    format_euro(ee.brutto)
                                } else {
                                    format_euro(0.0)
                                }
                            }
                            on:change=change_ee1_brutto
                        />
                    </td>
                </tr>
                <tr>
                    <td class="px-1">
                        "Einkommen netto:"
                    </td>
                    <td class="px-1">
                        <input type="text" min="0.0" class="px-1 border-2 border-stone-400 rounded-lg text-right" value={ move || format_euro(erwachsene_einkommen.get()[0].netto) } prop:value={ move || format_euro(erwachsene_einkommen.get()[0].netto) } on:change=change_ee0_netto />
                    </td>
                    <td class={ move || if couple.get() { "visible" } else { "hidden" }}>
                        <input 
                            type="text"
                            min="0.0"
                            class="px-1 border-2 border-stone-400 rounded-lg text-right"
                            value={
                                move || if let Some(ee) = erwachsene_einkommen.get().get(1) {
                                    format_euro(ee.netto)
                                } else {
                                    format_euro(0.0)
                                }
                            }
                            prop:value={
                                move || if let Some(ee) = erwachsene_einkommen.get().get(1) {
                                    format_euro(ee.netto)
                                } else {
                                    format_euro(0.0)
                                }                                
                            }
                            on:change=change_ee1_netto
                        />
                    </td>
                </tr>
                <tr>
                    <td class="px-1">
                        "Anrechenbares Einkommen:"
                    </td>
                    <td class="px-1">
                        <input type="text" min="0.0" class="px-1 border-2 border-stone-400 rounded-lg text-right" value={ move || format_euro(erwachsene_einkommen.get()[0].netto) } prop:value={ move || format_euro(erwachsene_einkommen.get()[0].netto) } on:change=change_ee0_netto />
                    </td>
                    <td class={ move || if couple.get() { "visible" } else { "hidden" }}>
                        <input 
                            type="text"
                            min="0.0"
                            class="px-1 border-2 border-stone-400 rounded-lg text-right"
                            value={
                                move || if let Some(ee) = erwachsene_einkommen.get().get(1) {
                                    format_euro(ee.netto)
                                } else {
                                    format_euro(0.0)
                                }
                            }
                            prop:value={
                                move || if let Some(ai) = erwachsene_einkommen.get().get(1) {
                                    format_euro(ai.netto)
                                } else {
                                    format_euro(0.0)
                                }                                
                            }
                            on:change=change_ee1_netto
                        />
                    </td>
                </tr>
            </table>
            <h3 class={ move || if k.get() > 0 { "text-xl font-medium" } else { "hidden" }}>
                "Einkommen der Kinder"
            </h3>
            <table class={ move || if k.get() > 0 { "visible" } else { "hidden" }}>
                <tr>
                    <td class="px-1">
                        "Einkommen brutto:"
                    </td>
                    <td class="px-1">
                        <input type="text" min="0.0" value={ move || format_euro(kinder_einkommen.get()[0].brutto) } class="border-2 border-stone-400 rounded-lg px-1" on:change=change_ke />
                    </td>
                </tr>
            </table>
        </div>
    }
}