use leptos::prelude::*;
use leptos_router::{hooks::query_signal_with_options, location::State, NavigateOptions};

mod defaults;
mod components;
use crate::components::{
    intro::Intro,
    community::Community,
    needs::Needs,
    notes::Notes
};

#[component]
pub fn LUSR() -> impl IntoView {
    let (p, set_p) = query_signal_with_options::<u32>(
        "p",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );

    view! {
        <Intro />
        <Community p=p set_p=set_p />
        <Needs p=p />
        <Notes />
    }
}