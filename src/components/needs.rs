use leptos::prelude::*;

use crate::utils::format_euro;
use crate::defaults;
use crate::standardrates::{
    ADULT_SINGLE,
    ADULT_COUPLE,
    U25,
    U18,
    U14,
    U6
};

// Bedarfe

#[component]
pub fn Needs(
    a: Memo<Option<u32>>,
    u25: Memo<Option<u32>>,
    u18: Memo<Option<u32>>,
    u14: Memo<Option<u32>>,
    u6: Memo<Option<u32>>,
    sn: Memo<f64>
) -> impl IntoView {

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Bedarfe"
            </h2>
            <table>
                <tr>
                    <td class="px-1">"Erwachsene:"</td>
                    {
                        move || if a.get().unwrap_or(defaults::ADULTS) == defaults::ADULTS {
                            view! {
                                <td class="px-1 text-right">"1 x "{ format_euro(ADULT_SINGLE) }</td>
                                <td class="px-1 text-right">{ format_euro(ADULT_SINGLE) }</td>
                            }
                        } else {
                            view! {
                                <td class="px-1 text-right">"2 x "{ format_euro(ADULT_COUPLE / 2.0) }</td>
                                <td class="px-1 text-right">{ format_euro(ADULT_COUPLE) }</td>
                            }
                        }
                    }
                </tr>
                <tr>
                    <td class="px-1">"Anzahl Erwachsene unter 25, die im elterlichen Haushalt leben:"</td>
                    <td class="px-1 text-right">
                        { move || u25.get().unwrap_or(defaults::U25) }
                        " x "
                        { format_euro(U25) }
                    </td>
                    <td class="px-1 text-right">
                        { move || format_euro(u25.get().unwrap_or(defaults::U25) as f64 * U25) }
                    </td>
                </tr>
                <tr>
                    <td class="px-1">"Anzahl Jugendliche von 14 bis 17 Jahren:"</td>
                    <td class="px-1 text-right">
                        { move || u18.get().unwrap_or(defaults::U18) }
                        " x "
                        { format_euro(U18) }
                    </td>
                    <td class="px-1 text-right">
                        { move || format_euro(u18.get().unwrap_or(defaults::U18) as f64 * U18) }
                    </td>
                </tr>
                <tr>
                    <td class="px-1">"Anzahl Kinder von 6 bis 13 Jahren:"</td>
                    <td class="px-1 text-right">
                        { move || u14.get().unwrap_or(defaults::U14) }
                        " x "
                        { format_euro(U14) }
                    </td>
                    <td class="px-1 text-right">
                        { move || format_euro(u14.get().unwrap_or(defaults::U14) as f64 * U14) }
                    </td>
                </tr>
                <tr>
                    <td class="px-1">"Anzahl Kinder von 0 bis 5 Jahren:"</td>
                    <td class="px-1 text-right">
                        { move || u6.get().unwrap_or(defaults::U6) }
                        " x "
                        { format_euro(U6) }
                    </td>
                    <td class="px-1 text-right">
                        { move || format_euro(u6.get().unwrap_or(defaults::U6) as f64 * U6) }
                    </td>
                </tr>
                <tr class="font-semibold">
                    <td class="px-1">"Summe der Regelbedarfe:"</td>
                    <td></td>
                    <td class="px-1 text-right">{ move || format_euro(sn.get()) }</td>
                </tr>
            </table>
        </div>
    }
}