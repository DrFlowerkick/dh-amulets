// home route of app

use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h2 class="text-2xl text-center md:text-left font-semibold tracking-wide text-secondary mb-4">
            "Willkommen!"
        </h2>
        <div class="text-base text-content mb-2 space-y-2">
            <p>
                "Dies ist ein nichtkommerzielles Fanprojekt für das Setup von "
                <a
                    href="https://www.kosmos.de/de/drachenhuter_1683757_4002051683757"
                    class="link link-primary"
                >
                    "Drachenhüter"
                </a> ", einem Kartenspiel erschienen im "
                <a href="https://www.kosmos.de" class="link link-primary">
                    "Kosmosverlag"
                </a> "."
            </p>
            <ul class="list-disc pl-5 text-base-content">
                <li>"Wähle im Navigationsmenü die Anzahl der Spieler aus."</li>
                <li>
                    "Es wird eine zufällige Kombination an Amuletten berechnet, die aus dem Vorrat entfernt werden müssen."
                </li>
                <li>
                    "Klicke oder tippe auf die Setup Überschrift für ein neues Setup bei der aktuellen Anzahl der Spieler."
                </li>
                <li>"Des Weiteren führt jeder Reload der Webseite zu einem neuen Setup."</li>
                <li>"Fertig."</li>
            </ul>
            <p>"Viel Spaß beim Spielen 😊"</p>
        </div>
        <h2 class="text-2xl text-center md:text-left font-semibold tracking-wide text-secondary mb-4">
            "Installation"
        </h2>
        <div class="text-base text-content mb-2 space-y-2">
            <p>
                <strong>"Drachenhüter Amulett Setup"</strong>
                " ist als "
                <a
                    href="https://de.wikipedia.org/wiki/Progressive_Webanwendung"
                    class="link link-primary"
                >
                    "Progressive Webanwendung (PWA)"
                </a>
                " konzipiert. Somit kann die App sowohl mit allen Browsern, die PWA unterstützen, als auch unter Android lokal
                installiert werden."
            </p>
        </div>
        <h3 class="text-xl text-center md:text-left font-semibold tracking-wide text-secondary mb-4">
            "Installation in Android"
        </h3>
        <div class="text-base text-content mb-2 space-y-2">
            <p>
                "Um die App unter Android zu installieren, muss diese zum "
                <strong>"Startbildschirm"</strong> " hinzugefügt werden:"
            </p>
            <ul class="list-disc pl-5 text-base-content">
                <li>
                    <strong>"firefox"</strong>
                    " Tippe auf die drei Punkte, danach \"App zum Startbildschirm hinzufügen\" auswählen."
                </li>
                <li>
                    <strong>"vivaldi"</strong>
                    " Tippe auf das vivaldi icon, danach \"Seite hinzufügen zu ...\" auswählen
                    und dann Startbildschirm wählen."
                </li>
            </ul>
            <p>
                "In anderen Browsern wird das Vorgehen wahrscheinlich ähnlich sein, wurde aber bisher nicht getestet."
            </p>
        </div>
        <h3 class="text-xl text-center md:text-left font-semibold tracking-wide text-secondary mb-4">
            "Installation in Linux und Windows"
        </h3>
        <div class="text-base text-content mb-2 space-y-2">
            <p>
                "Um die App in Linux und Windows zu installieren, muss die Webseite in einem Browser geöffnet werden, der PWA unterstützt.
                Danach dann wie folgt weiter vorgehen:"
            </p>
            <ul class="list-disc pl-5 text-base-content">
                <li>
                    <strong>"chromium"</strong>
                    " Links-click auf das "
                    <em>"Monitor Symbol mit dem Pfeil nach unten"</em>
                    " rechts in der
                    URL Adressleiste. Wenn man mit der Maus über dem Symbol schwebt, wird der Informationstext \"Drachenhüter Amulet
                    Setup installieren\" angezeigt."
                </li>
                <li>
                    <strong>"vivaldi"</strong>
                    " Rechts-click in der Tableiste auf den Tab mit der geöffneten Drachenhüter
                    Webseite und \"Progressive Web-Apps\" -> \"Drachenhüter Amulet Setup installieren...\" auswählen."
                </li>
            </ul>
            <p>
                "Chrome und Edge wurden bisher nicht getestet, sollten aber wie bei chromium funktionieren. Leider unterstützt firefox
                kein PWA unter Linux und Windows. Verwende hierfür einen chromium basierten Browser."
            </p>
        </div>
        <h2 class="text-2xl text-center md:text-left font-semibold tracking-wide text-secondary mb-4">
            "Themenauswahl"
        </h2>
        <div class="text-base text-content mb-2 space-y-2">
            <p>
                "Du kannst verschiedene Themen für "<strong>"Drachenhüter Amulett Setup"</strong>
                " auswählen. Probiere sie einfach aus. Klicke dazu den "<em>"Menü"</em>
                " Button oben rechts."
            </p>
        </div>
    }
}
