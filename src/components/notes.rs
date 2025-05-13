use leptos::prelude::*;

// Rechtliche Hinweise

#[component]
pub fn Notes() -> impl IntoView {
    view! {
        <div class="container max-w-screen-xl mx-auto px-4 bg-linear-to-b from-stone-50 to-stone-300">
            <h2 class="pt-4 text-2xl font-medium">
                "Rechtliche Hinweise"
            </h2>
            <p>
                "Der Rechner geht äußerst sparsam mit deinen Daten um. Zwar werden einige technisch benötigte Daten,
                insbesondere deine IP-Adresse, an meinen Server gesendet und von meinem Server verarbeitet. Das ist nötig,
                um die zum Rechner gehörenden Dateien an deinen Browser oder dein sonstiges Gerät, mit welchem du den
                Rechner ausführst, schicken zu können. Der Rechner wird aber vollständig auf deinem Gerät ausgeführt. Das
                bedeutet, dass alle Daten, die du in den Rechner eingibst, und die Ergebnisse, die von meinem Rechner
                erzeugt werden, vollständig bei dir verbleiben, und erst gar nicht an meinen Server geschickt, geschweige
                denn verarbeitet oder gespeichert werden."
            </p>
            <p>
                "Der Rechner ist zudem auch als Freie Software unter den Lizenzen Apache, Version 2.0, und MIT
                veröffentlicht. Du kannst dir die Software also auch aus dem "<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://github.com/dusmarcel/mpkr25">
                "Repository"</a>" herunterladen und sie dann ganz auf einem Gerät deiner Wahl ausführen.
                In diesem Falle hast du mit meinem Server gar nichts mehr zu tun, und die Notwendigkeit, Daten an meinen
                Server zu übertragen, entfällt ganz."
            </p>
            <p>
                "Und schließlich geht es hier noch zu meinem „"<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://aufentha.lt/index.php/impressum/">"Impressum"</a>"“."
            </p>
        </div>
    }
}