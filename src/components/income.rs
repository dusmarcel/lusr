use leptos::prelude::*;
use reactive_stores::Store;
//use leptos::web_sys::console;

use crate::{
    defaults, incomes::{
        adults::{erwachsene_einkommen_to_string, ErwachsenEinkommen},
        children::{kinder_einkommen_to_string, KindEinkommen}
    },
    standardrates::KINDERGELD,
    utils::{anr_einkommen, format_euro}
};

// Einkommen

#[component]
pub fn Income(
    e: Memo<Option<u32>>,
    k: Memo<u32>,
    f: Memo<Option<bool>>,
    set_f: SignalSetter<Option<bool>>,
    erwachsene_einkommen: Memo<Vec<ErwachsenEinkommen>>,
    set_ee: SignalSetter<Option<String>>,
    kinder_einkommen: Memo<Vec<KindEinkommen>>,
    set_ke: SignalSetter<Option<String>>,
    set_ae: WriteSignal<f64>
) -> impl IntoView {
    let change_freibetraege = move |e| set_f.set(Some(event_target_checked(&e)));

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
    let change_ee0_sonstige = move |e| {
        let mut new_adults_incomes = erwachsene_einkommen.get().clone();
        new_adults_incomes[0].sonstige = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
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
    let change_ee1_sonstige = move |e| {
        let mut new_adults_incomes = erwachsene_einkommen.get().clone();
        new_adults_incomes[1].sonstige = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
        set_ee.set(Some(erwachsene_einkommen_to_string(&new_adults_incomes)));
    };

    Effect::new( move |_| {
        let mut summe = 0.0;
        summe += anr_einkommen(erwachsene_einkommen.get()[0].netto) + erwachsene_einkommen.get()[0].sonstige;
        if let Some(ee) = erwachsene_einkommen.get().get(1) {
            summe += anr_einkommen(ee.netto) + ee.sonstige;
        }
        for kind in kinder_einkommen.get() {
            if kind.id < k.get() as usize {
                summe += anr_einkommen(kind.netto) + kind.kinderzuschlag + kind.sonstige;
                if kind.kindergeld { summe += KINDERGELD as f64 }
            }
        }
        set_ae.set(summe);
    });

    let data = Store::new(move || kinder_einkommen.get());

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Einkommen"
            </h2>
            <p>
                "Dem ermittelten Bedarf müssen wir jetzt das Einkommen der Bedarfsgemeinschaft gegenüberstellen."
            </p>
            <p>
                <input
                    type="checkbox"
                    id="freibetraege"
                    on:change=change_freibetraege
                    prop:checked=move || f.get().unwrap_or(true)
                />
                <label for="freibetraege" class="ml-1">"Absetzbeträge gemäß § 11b Abs. 1 Nr. 6 i.V.m. Abs. 3 SGB II berücksichtigen"</label>
                <button popovertarget="aussergerichtliche-vertretung" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                <div id="aussergerichtliche-vertretung" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg">
                    <h4 class="text-xl font-medium">"Absetzbeträge"</h4>
                    <p>"Nach der Rechtsprechung sind vom Netto-Einkommen bestimmte Freibeträge gemäß § 11b SGB II abzusetzen.
                    Der Rechner nimmt diese Abzüge automatisch vor. Im Anwendungsbereich der Familienzusammenführungsrichtlinie
                    (Richtlinie 2003/86/EG) entfällt jedoch ein Teil dieser Abszüge. Daher kann du das Häkchen hier entfernen,
                    wenn ein Aufenthaltstitel zum Familiennachzug von Ehegatt*in, Lebenspartner*in, oder minderjähirigen Kindern
                    zu Ausländer*innen, die ihrerseits einen Aufenthaltstitel mit einer Gültigkeit von mindestens einem Jahr besitzen."</p>
                    <p>"Nach richtiger Auffassung gilt dasselbe, wenn eine Erlaubnis zum Daueraufenthalt-EU (§ 9a AufenthG) angestrebt wird."</p>
                    <p>"Wenn du dir unsicher bist, lasse das Häkchen gesetzt."</p>
                    <p>"Die Werbungskostenpauschale (§ 11b Abs. 2 SGB II wird von diesem Rechner stets vom Netto-Einkommen
                    abgesetzt. Daher ist das anrechende Einkommen stets niedriger als das Netto-Einkommen. Es wäre ggf. möglich,
                    hier noch einen weiteren Schalter einzubauen, der es ermöglicht, auch die Anrechnung der Werbungskostenpauschale
                    zu ermöglichen (oder auch, hierfür einen anderen, frei wählbaren Betrag, anzugeben, sofern hierfür ein praktisches
                    Bedürfnis besteht. Ich freue mich über Feedback!"</p>
                </div>   
            </p>
            <table>
                <thead>
                    <tr>
                        <th class="px-1 text-left">
                            "Einkommen brutto"
                        </th>
                        <th class="px-1 text-left">
                            "Einkommen netto"
                        </th>
                        <th class="px-1 text-left">
                            "Sonstige Einkünfte"
                            <button popovertarget="sonstige-einkuenfte" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                            <div id="sonstige-einkuenfte" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg open:text-left">
                                <h4 class="text-xl font-medium">"Sonstige Einkünfte"</h4>
                                <p class="font-normal">"In Betracht kommen beispielsweise Erziehungsgeld nach Landesrecht,
                                Betreuungs-, Familien- und Elterngeld sowie Renten, Stipendien, Krankengeld und Ausbildungsbeihilfen wie bspw.
                                BAföG oder auch ALG I."</p>
                                <p class="font-normal">"Kindergeld und Kingerzuschlag gehören nicht hierher, sondern können unten bei den Kindern angegeben werden."</p>
                                <p class="font-normal">"Wohngeld, Unterhaltsvorschusszahlungen und Pflegegeld sind hier ebenfalls nicht anzugeben. Zwar sind diese Leistungen unschädlich,
                                sie dürfen aber auch nicht als Einkommen angerechnet werden."</p>
                            </div>  
                        </th>
                        <th class="px-1 text-left">
                            "Anrechenbares Einkommen"
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr>                
                        <td class="px-1">
                            <input type="text" min="0.0" class="px-1 border-2 border-stone-400 rounded-lg text-right" value={ move || format_euro(erwachsene_einkommen.get()[0].brutto) } prop:value={ move || format_euro(erwachsene_einkommen.get()[0].brutto) } on:change=change_ee0_brutto />
                        </td>
                        <td class="px-1">
                            <input type="text" min="0.0" class="px-1 border-2 border-stone-400 rounded-lg text-right" value={ move || format_euro(erwachsene_einkommen.get()[0].netto) } prop:value={ move || format_euro(erwachsene_einkommen.get()[0].netto) } on:change=change_ee0_netto />
                        </td>
                        <td class="px-1">
                            <input type="text" min="0.0" class="px-1 border-2 border-stone-400 rounded-lg text-right" value={ move || format_euro(erwachsene_einkommen.get()[0].sonstige) } prop:value={ move || format_euro(erwachsene_einkommen.get()[0].sonstige) } on:change=change_ee0_sonstige />
                        </td>
                        <td class="px-1 text-right">
                            { move || format_euro(anr_einkommen(erwachsene_einkommen.get()[0].netto) + erwachsene_einkommen.get()[0].sonstige) }
                        </td>
                    </tr>
                    <tr class={ move || if couple.get() { "visible" } else { "hidden" }}>
                        <td class="px-1">
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
                        <td class="px-1">
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
                        <td class="px-1">
                            <input 
                                type="text"
                                min="0.0"
                                class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                value={
                                    move || if let Some(ee) = erwachsene_einkommen.get().get(1) {
                                        format_euro(ee.sonstige)
                                    } else {
                                        format_euro(0.0)
                                    }
                                }
                                prop:value={
                                    move || if let Some(ee) = erwachsene_einkommen.get().get(1) {
                                        format_euro(ee.sonstige)
                                    } else {
                                        format_euro(0.0)
                                    }                                
                                }
                                on:change=change_ee1_sonstige
                            />
                        </td>
                        <td class="px-1 text-right">
                            {
                                move || if let Some(ee) = erwachsene_einkommen.get().get(1) {
                                    format_euro(anr_einkommen(ee.netto) + ee.sonstige)
                                } else {
                                    format_euro(0.0)
                                }
                            }
                        </td>
                    </tr>
                </tbody>
            </table>
            <h3 class={ move || if k.get() > 0 { "text-xl font-medium" } else { "hidden" }}>
                "Einkommen der Kinder"
            </h3>
            <table class={ move || if k.get() > 0 { "visible" } else { "hidden" }}>
                <thead>
                    <tr>
                        <th class="px-1 text-left">
                            "Einkommen brutto"
                        </th>
                        <th class="px-1 text-left">
                            "Einkommen netto"
                        </th>
                        <th class="px-1 text-left">
                            "Kindergeld"
                        </th>
                        <th class="px-1 text-left">
                            "Kinderzuschlag"
                        </th>
                        <th class="px-1 text-left">
                            "Sonstige Einkünfte"
                            <button popovertarget="sonstige-einkuenfte-kinder" class="px-1 ml-1 border-2 border-stone-400 rounded-lg">?</button>
                            <div id="sonstige-einkuenfte-kinder" popover class="open:fixed open:left-1/2 open:top-1/4 open:-translate-x-1/2 open:max-w-lg open:w-full open:px-4 open:z-50 open:border-2 open:border-stone-400 open:rounded-lg open:bg-white open:shadow-lg open:text-left">
                                <h4 class="text-xl font-medium">"Sonstige Einkünfte"</h4>
                                <p class="font-normal">"In Betracht kommen beispielsweise Erziehungsgeld nach Landesrecht,
                                Betreuungs-, Familien- und Elterngeld sowie Renten, Stipendien, Krankengeld und Ausbildungsbeihilfen wie bspw. BAföG oder auch ALG I."</p>
                                <p class="font-normal">"Wohngeld, Unterhaltsvorschusszahlungen und Pflegegeld sind hier nicht anzugeben. Zwar sind diese Leistungen unschädlich,
                                sie dürfen aber auch nicht als Einkommen angerechnet werden."</p>
                            </div>  
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
                    <tr class={ move || if k.get() as usize > kind.id { "visible" } else { "hidden" }}>
                        <td class="px-1">
                            <input
                                type="text"
                                min="0.0"
                                class="px-1 border-2 border-stone-400 rounded-lg text-right" 
                                value={
                                    move || if let Some(kind) = kinder_einkommen.get().get(kind.id) {
                                        format_euro(kind.brutto)
                                    } else {
                                        format_euro(0.0)
                                    }
                                }
                                prop:value={
                                    move || if let Some(kind) = kinder_einkommen.get().get(kind.id) {
                                        format_euro(kind.brutto)
                                    } else {
                                        format_euro(0.0)
                                    }
                                }
                                on:change={ move |e| { 
                                    let mut nci = kinder_einkommen.get().clone();
                                    nci[kind.id].brutto = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
                                    set_ke.set(Some(kinder_einkommen_to_string(&nci)));
                                }}
                            />
                        </td>
                        <td class="px-1">
                            <input
                                type="text"
                                min="0.0"
                                class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                value={
                                    move || if let Some(kind) = kinder_einkommen.get().get(kind.id) {
                                        format_euro(kind.netto)
                                    } else {
                                        format_euro(0.0)
                                    }
                                }
                                prop:value={
                                    move || if let Some(kind) = kinder_einkommen.get().get(kind.id) {
                                        format_euro(kind.netto)
                                    } else {
                                        format_euro(0.0)
                                    }
                                }                                
                                on:change={move |e| { 
                                    let mut nci = kinder_einkommen.get().clone();
                                    nci[kind.id].netto = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
                                    set_ke.set(Some(kinder_einkommen_to_string(&nci)));
                                }}
                            />
                        </td>
                        <td class="px-1 text-center">
                            <input
                                type="checkbox"
                                checked={
                                    move || if let Some(kind) = kinder_einkommen.get().get(kind.id) {
                                        kind.kindergeld
                                    } else {
                                        false
                                    }
                                }
                                prop:checked={
                                    move || if let Some(kind) = kinder_einkommen.get().get(kind.id) {
                                        kind.kindergeld
                                    } else {
                                        false
                                    }
                                }
                                on:change={move |e| {
                                    let mut nci = kinder_einkommen.get().clone();
                                    nci[kind.id].kindergeld = event_target_checked(&e);
                                    set_ke.set(Some(kinder_einkommen_to_string(&nci)));
                                }}
                            />
                        </td>
                        <td class="px-1">
                            <input
                                type="text"
                                min="0.0"
                                class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                value={
                                    move || if let Some(kind) = kinder_einkommen.get().get(kind.id) {
                                        format_euro(kind.kinderzuschlag)
                                    } else {
                                        format_euro(0.0)
                                    }
                                }
                                prop:value={
                                    move || if let Some(kind) = kinder_einkommen.get().get(kind.id) {
                                        format_euro(kind.kinderzuschlag)
                                    } else {
                                        format_euro(0.0)
                                    }
                                }                                
                                on:change={move |e| { 
                                    let mut nci = kinder_einkommen.get().clone();
                                    nci[kind.id].kinderzuschlag = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
                                    set_ke.set(Some(kinder_einkommen_to_string(&nci)));
                                }}
                            />
                        </td>
                        <td class="px-1">
                            <input
                                type="text"
                                min="0.0"
                                class="px-1 border-2 border-stone-400 rounded-lg text-right"
                                value={
                                    move || if let Some(kind) = kinder_einkommen.get().get(kind.id) {
                                        format_euro(kind.sonstige)
                                    } else {
                                        format_euro(0.0)
                                    }
                                }
                                prop:value={
                                    move || if let Some(kind) = kinder_einkommen.get().get(kind.id) {
                                        format_euro(kind.sonstige)
                                    } else {
                                        format_euro(0.0)
                                    }
                                }                                
                                on:change={move |e| { 
                                    let mut nci = kinder_einkommen.get().clone();
                                    nci[kind.id].sonstige = event_target_value(&e).parse::<f64>().unwrap_or(0.0);
                                    set_ke.set(Some(kinder_einkommen_to_string(&nci)));
                                }}
                            />
                        </td>
                        <td class="px-1 text-right">
                            {
                                move || if let Some(kind) = kinder_einkommen.get().get(kind.id) {
                                    let mut e = anr_einkommen(kind.netto) + kind.kinderzuschlag + kind.sonstige;
                                    if kind.kindergeld { e += KINDERGELD as f64 }
                                    format_euro(e)
                                } else {
                                    format_euro(0.0)
                                }
                            }
                        </td>
                    </tr>
                </For>
                </tbody>
            </table>
        </div>
    }
}