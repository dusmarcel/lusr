use leptos::prelude::*;

use crate::{
    utils::format_euro,
    incomes::{
        adults::{AdultIncome, adults_incomes_to_string},
        children::{ChildIncome, children_incomes_to_string}
    }
};

// Einkommen

#[component]
pub fn Income(
    a: Memo<Option<u32>>,
    c: Memo<u32>,
    adults_incomes: Memo<Vec<AdultIncome>>,
    set_ai: SignalSetter<Option<String>>,
    children_incomes: Memo<Vec<ChildIncome>>,
    set_ci: SignalSetter<Option<String>>
) -> impl IntoView {
    let change_ai = move |e| set_ai.set(Some(adults_incomes_to_string(&adults_incomes.get())));
    let change_ci = move |e| set_ci.set(Some(children_incomes_to_string(&children_incomes.get())));

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
                        <input type="number" min="0.0" value=move || format_euro(adults_incomes.get()[0].brutto) class="border-2 border-stone-400 rounded-lg px-1" on:change=change_ai />
                    </td>
                </tr>
            </table>
            <h3 class="text-xl font-medium">
                "Einkommen der Kinder"
            </h3>
            <table>
                <tr>
                    <td class="px-1">
                        "Einkommen brutto:"
                    </td>
                    <td class="px-1">
                        <input type="number" min="0.0" value=move || format_euro(children_incomes.get()[0].brutto) class="border-2 border-stone-400 rounded-lg px-1" on:change=change_ci />
                    </td>
                </tr>
            </table>            
        </div>
    }
}