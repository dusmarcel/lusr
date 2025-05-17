use leptos::prelude::*;

// Ergebnis

#[component]
pub fn Result(

) -> impl IntoView {

    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Ergebnis"
            </h2>
            <p>
                "Ist der Lebensunterhalt gesichert?"
            </p>
        </div>
    }
}