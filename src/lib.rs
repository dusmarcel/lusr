use leptos::prelude::*;

pub mod components;
use crate::components::{intro::Intro, notes::Notes};

#[component]
pub fn LUSR() -> impl IntoView {
    view! {
        <Intro />
        <Notes />
    }
}