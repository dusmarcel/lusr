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
                "Dieser Rechner soll die "<span class="font-semibold">"aufenthaltsrechtliche Beratung"</span>" zur Frage, ob der Lebensunterhalt im Sinne der "
                <a class="text-blue-600 hover:underline hover:text-violet-600" href="https://dejure.org/gesetze/AufenthG/5.html">
                "§§ 2 Abs. 3, 5 Abs. 1 Nr. 1 AufenthG"</a>" gesichert werden kann, unterstützen. Dabei kann und soll er die Beratung
                bestenfalls begleiten, aber keinesfalls ersetzen, da es, je nach konkretem Fall, eine Vielzahl von Besonderheiten zu beachten
                geben kann, die hier ummöglich alle abgebildet werden können."
            </p>
            <p>
                "Auch ist wichtig, zu betonen, dass dieser Rechner sich als Tool für die aufenthaltsrechtliche Beratung, also für
                die Auseinandersetzung mit der Ausländerbehörde (bzw. in entsprechenden Verfahren mit dem Verwaltungsgericht oder der
                deutschen Auslandsvertretung), aber "<span class="font-semibold"><span class="underline">"nicht"</span>" für die sozialrechtliche Beratung"</span>" im Zusammenhang mit der Auseinandersetzung
                mit Sozialämtern, Sozialgerichten oder Jobcentern versteht. Hierfür findet sich im Netz eine Vielzahl anderer,
                geeigneterer Angebote in Gestalt von Bürgergeldrechnern und ähnlichen Dingen."
            </p>
            <p>
                "Die Werbungskostenpauschale ("
                <a class="text-blue-600 hover:underline hover:text-violet-600" href="https://dejure.org/gesetze/SGB_II/11b.html">"§ 11b Abs. 2 SGB II"</a>
                ") wird von diesem Rechner stets vom Netto-Einkommen
                abgesetzt. Daher ist das anrechende Einkommen stets niedriger als das Netto-Einkommen. Es wäre ggf. möglich,
                hier noch einen weiteren Schalter einzubauen, der es ermöglicht, auch die Anrechnung der Werbungskostenpauschale
                zu ermöglichen (oder auch, hierfür einen anderen, frei wählbaren Betrag, anzugeben, sofern hierfür ein praktisches
                Bedürfnis besteht. Ich freue mich über Feedback!"
            </p>
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
                veröffentlicht. Du kannst dir die Software also auch aus dem "<a class="text-blue-600 hover:underline hover:text-violet-600" href="https://github.com/dusmarcel/lusr">
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