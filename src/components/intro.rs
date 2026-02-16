use leptos::prelude::*;

// Intro

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h1 class="pt-4 text-3xl font-medium">
                "Lebensunterhaltssicherungsrechner"
            </h1>
            <p>
                "Erstellt von "<a class="text-blue-800 hover:underline hover:text-blue-900" href="https://aufentha.lt">"Marcel Keienborg"</a>". Bitte beachte unbedingt auch die
                Hinweise unten auf dieser Seite."
            </p>
            <p>
                <span class="underline font-semibold text-red-700">"!!!Achtung!!!"</span>" Dieser Rechner ist aktuell noch eine Baustelle. Von einem produktiven Einsatz in der Beratung wird derzeit noch ausdrücklich abgeraten. Er wird derzeit ausschließlich zu Testzwecken veröffentlicht, mit der Bitte um Feedback und Hinweisen zu Fehlern, Verbesserungsvorschlägen etc. gerne per E-Mail an "<a class="text-blue-800 hover:underline hover:text-blue-900" href="mailto:marcel@aufentha.lt">"marcel@aufentha.lt"</a>"."
            </p>
        </div>
    }
}
