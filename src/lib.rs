use incomes::children::{kinder_einkommen_to_string, KindEinkommen};
use leptos::prelude::*;
use leptos_router::{hooks::query_signal_with_options, location::State, NavigateOptions};
//use leptos::web_sys::console;

mod utils;
mod defaults;
mod standardrates;
use crate::standardrates::{
    ERWACHSEN_SINGLE,
    ERWACHSEN_PAAR,
    U25,
    U18,
    U14,
    U6
};
mod incomes;
use crate::incomes::{
    children::kinder_einkommen_from_string,
    adults::{ErwachsenEinkommen, erwachsene_einkommen_from_string, erwachsene_einkommen_to_string}
};
mod components;
use crate::components::{
    intro::Intro,
    community::Community,
    needs::Needs,
    income::Income,
    result::Result,
    notes::Notes,
    calc::Calculations
};

#[component]
pub fn LUSR() -> impl IntoView {
    // adult person(s) in the community (Bedarfsgemeinschaft)
    // should only be 1 (default) or 2
    let (e, set_e) = query_signal_with_options::<u32>(
        "e",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );
    if e.get_untracked().unwrap_or(defaults::ERWACHSENE) != 1 && e.get_untracked().unwrap_or(defaults::ERWACHSENE) != 2 { set_e.set(Some(defaults::ERWACHSENE)) };

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
    let (m, set_m) = query_signal_with_options::<f64>(
        "m",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );

    // the health insurance (Krankenversicherung)
    let (kv, set_kv) = query_signal_with_options::<f64>(
        "kv",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );    

    // the child maintenance (Kindesunterhalt)
    let (ku, set_ku) = query_signal_with_options::<f64>(
        "ku",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );

    // Calculate the sum of the Needs (Bedarfe)
    let sb = Memo::new(move |_| {
        let mut summe = 0.0;
        if e.get().unwrap_or(defaults::ERWACHSENE) == 1 {
            summe += ERWACHSEN_SINGLE
        } else {
            summe += ERWACHSEN_PAAR
        }
        summe += u25.get().unwrap_or(defaults::U25)  as f64 * U25;
        summe += u18.get().unwrap_or(defaults::U18) as f64 * U18;
        summe += u14.get().unwrap_or(defaults::U14) as f64 * U14;
        summe += u6.get().unwrap_or(defaults::U6) as f64 * U6;
        summe += m.get().unwrap_or(defaults::MIETE);
        summe += kv.get().unwrap_or(defaults::KV);
        summe += ku.get().unwrap_or(defaults::KU);
        summe
    });

    let (f, set_f) = query_signal_with_options::<bool>(
        "f",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) });

    let (ee, set_ee) = query_signal_with_options::<String>(
        "ee",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );
    let mut eek = erwachsene_einkommen_from_string(ee.get_untracked().unwrap_or_default());
    if eek.len() < 2 {
        eek.push(ErwachsenEinkommen::new());
        set_ee.set(Some(erwachsene_einkommen_to_string(&eek)));
    }
    let erwachsene_einkommen = Memo::new( move |_| erwachsene_einkommen_from_string(ee.get().unwrap_or_default()));
    
    let (ke, set_ke) = query_signal_with_options::<String>(
        "ke",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );
    let kinder_einkommen = Memo::new( move |_| kinder_einkommen_from_string(ke.get().unwrap_or_default()));

    // Calculate number of children
    let k = Memo::new( move |_| {
        let mut lke = kinder_einkommen_from_string(ke.get().unwrap_or_default());
        let mut k = 0;
        k += u25.get().unwrap_or(defaults::U25);
        k += u18.get().unwrap_or(defaults::U18);
        k += u14.get().unwrap_or(defaults::U14);
        k += u6.get().unwrap_or(defaults::U6);
        while lke.len() < k as usize {
            lke.push(KindEinkommen::new(lke.len()));
        }
        set_ke.set(Some(kinder_einkommen_to_string(&lke)));
        k
    });
    let mj_kinder = Memo::new(move |_| u18.get().unwrap_or(defaults::U18) + u14.get().unwrap_or(defaults::U14) + u6.get().unwrap_or(defaults::U6));

    let (ae, set_ae) = signal(0.0);

    view! {
        <Intro />
        <Community e=e set_e=set_e u25=u25 set_u25=set_u25 u18=u18 set_u18=set_u18 u14=u14 set_u14=set_u14 u6=u6 set_u6=set_u6 m=m set_m=set_m kv=kv set_kv=set_kv ku=ku set_ku=set_ku />
        <Needs e=e u25=u25 u18=u18 u14=u14 u6=u6 m=m kv=kv ku=ku sb=sb />
        <Income e=e k=k mj_kinder=mj_kinder f=f set_f=set_f erwachsene_einkommen=erwachsene_einkommen set_ee=set_ee kinder_einkommen=kinder_einkommen set_ke=set_ke set_ae=set_ae />
        <Result sb=sb ae=ae />
        <Calculations e=e k=k mj_kinder=mj_kinder f=f erwachsene_einkommen=erwachsene_einkommen kinder_einkommen=kinder_einkommen />
        <Notes />
    }
}