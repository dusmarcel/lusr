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
    // adult person(s) in the community (Bedarfsgemeinschaft)
    // should only be 1 (default) or 2
    let (a, set_a) = query_signal_with_options::<u32>(
        "a",
        NavigateOptions { resolve: true, replace: false, scroll: false, state: State::new(None) }
    );
    if a.get().unwrap_or(defaults::ADULTS) != 1 && a.get().unwrap_or(defaults::ADULTS) != 2 { set_a.set(Some(defaults::ADULTS)); }

    view! {
        <Intro />
        <Community a=a set_a=set_a />
        <Needs a=a />
        <Notes />
    }
}