use leptos::prelude::*;
use leptos_router::{hooks::query_signal_with_options, location::State, NavigateOptions};

mod utils;
mod defaults;
mod standardrates;
use crate::standardrates::{
    ADULT_SINGLE,
    ADULT_COUPLE,
    U25,
    U18,
    U14,
    U6
};
mod components;
use crate::components::{
    intro::Intro,
    community::Community,
    needs::Needs,
    notes::Notes
};

#[component]
pub fn LUSR() -> impl IntoView {
    // adult person(s) in the community (Bedarfsgemeinschaft)
    // should only be 1 (default) or 2
    let (a, set_a) = query_signal_with_options::<u32>(
        "a",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );
    if a.get().unwrap_or(defaults::ADULTS) != 1 && a.get().unwrap_or(defaults::ADULTS) != 2 { set_a.set(Some(defaults::ADULTS)); }

    // their children
    let (u25, set_u25) = query_signal_with_options::<u32>(
        "u25",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );
    let (u18, set_u18) = query_signal_with_options::<u32>(
        "u18",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );
    let (u14, set_u14) = query_signal_with_options::<u32>(
        "u14",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );
    let (u6, set_u6) = query_signal_with_options::<u32>(
        "u6",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );

    // the rent (Miete)
    let (r, set_r) = query_signal_with_options::<f64>
        ("r",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );
    
    // Calculate the sum of the Needs (Bedarfe)
    let sn = Memo::new(move |_| {
        let mut summe = 0.0;
        if a.get().unwrap_or(defaults::ADULTS) == 1 {
            summe += ADULT_SINGLE
        } else {
            summe += ADULT_COUPLE
        }
        summe += u25.get().unwrap_or(defaults::U25)  as f64 * U25;
        summe += u18.get().unwrap_or(defaults::U18) as f64 * U18;
        summe += u14.get().unwrap_or(defaults::U14) as f64 * U14;
        summe += u6.get().unwrap_or(defaults::U6) as f64 * U6;
        summe += r.get().unwrap_or(defaults::RENT);
        summe
    });

    view! {
        <Intro />
        <Community a=a set_a=set_a u25=u25 set_u25=set_u25 u18=u18 set_u18=set_u18 u14=u14 set_u14=set_u14 u6=u6 set_u6=set_u6 r=r set_r=set_r />
        <Needs a=a u25=u25 u18=u18 u14=u14 u6=u6 r=r sn=sn />
        <Notes />
    }
}