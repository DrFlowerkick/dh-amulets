use crate::amulets::*;
use leptos::prelude::*;
use leptos::web_sys;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{A, Route, Router, Routes},
    hooks::{use_navigate, use_params},
    params::Params,
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="de" data-theme="aqua">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <Stylesheet id="leptos" href="/pkg/dh-amulets.css" />
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="Drachenhüter Amulett Setup" />

        // define routes
        <Router>
            <header class="p-4 text-center bg-base-200 shadow-md">
                <h1 class="text-3xl font-bold tracking-wide text-primary">
                    "Drachenhüter Amulett Setup"
                </h1>
            </header>

            <nav class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-[0.2em] bg-base-300">
                <A href="/" attr:class="nav-btn">
                    "Home"
                </A>
                <A href="/setup/2" attr:class="nav-btn" attr:title="Setup für zwei Spieler">
                    <PawnIcon />
                    <PawnIcon />
                </A>
                <A href="/setup/3" attr:class="nav-btn" attr:title="Setup für drei Spieler">
                    <PawnIcon />
                    <PawnIcon />
                    <PawnIcon />
                </A>
                <A href="/setup/4" attr:class="nav-btn" attr:title="Setup für vier Spieler">
                    <PawnIcon />
                    <PawnIcon />
                    <PawnIcon />
                    <PawnIcon />
                </A>
            </nav>

            <main class="p-4">
                <Routes fallback=|| NotFoundView>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=path!("setup") view=RedirectToSetupDefault />
                    <Route path=path!("setup/:num") view=SetUp />
                    <Route path=path!("not-found") view=NotFoundView />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn PawnIcon() -> impl IntoView {
    view! {
        <svg
            class="nav-pawn"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="16 0 32 50"
            fill="currentColor"
        >
            <circle cx="32" cy="16" r="12" />
            <path d="M32 30c-9 0-16 7-16 16h32c0-9-7-16-16-16z" />
        </svg>
    }
}

/// Renders not found view
#[component]
fn NotFoundView() -> impl IntoView {
    "Page not found."
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
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
                </a>", einem Kartenspiel
                erschienen im "<a href="https://www.kosmos.de" class="link link-primary">
                    "Kosmosverlag"
                </a>.
            </p>
            <ul class="list-disc pl-5 text-base-content">
                <li>"Wähle im Navigationsmenü die Anzahl der Spieler aus."</li>
                <li>
                    "Es wird eine zufällige Kombination an Amuletten berechnet, die aus dem Vorrat entfernt werden müssen."
                </li>
                <li>
                    "Nutze den Button 'Neues Setup' für ein neues Setup bei der aktuellen Anzahl der Spieler."
                </li>
                <li>"Des Weiteren führt jeder Reload zu einem neuen Setup."</li>
                <li>"Fertig."</li>
            </ul>
            <p>"Viel Spaß beim Spielen 😊"</p>
        </div>
        {ThemeSelector()}
    }
}

#[component]
pub fn ThemeSelector() -> impl IntoView {
    let (theme, set_theme) = signal(None::<String>);

    // Get theme from document if signal is set to None
    Effect::new(move |_| {
        if theme.get().is_none() {
            if let Some(theme) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|doc| doc.document_element())
                .and_then(|el| el.get_attribute("data-theme"))
            {
                set_theme.set(Some(theme));
            }
        }
    });

    view! {
        <h2 class="text-2xl text-center md:text-left font-semibold tracking-wide text-secondary mb-4">
            "Themenauswahl"
        </h2>
        <div class="text-base text-content mb-2 space-y-2">
            <p>
                "Du kannst verschiedene Themen für "<strong>"Drachenhüter Amulett Setup"</strong>
                " auswählen. Probiere sie einfach aus."
            </p>
        </div>
        <button class="btn mx-auto md:mx-0 block" popovertarget="popover-1" style="anchor-name:--anchor-1">
            "Thema wählen (Aktuell: "
            {move || theme.get()}
            ")"
        </button>
        <ul
            class="dropdown menu w-52 rounded-box bg-base-100 shadow-sm"
            popover
            id="popover-1"
            style="position-anchor:--anchor-1"
        >
            {ThemeButton(ThemeButtonProps {
                theme_name: "fantasy",
                set_theme,
            })}
            {ThemeButton(ThemeButtonProps {
                theme_name: "caramellatte",
                set_theme,
            })}
            {ThemeButton(ThemeButtonProps {
                theme_name: "coffee",
                set_theme,
            })}
            {ThemeButton(ThemeButtonProps {
                theme_name: "business",
                set_theme,
            })}
            {ThemeButton(ThemeButtonProps {
                theme_name: "synthwave",
                set_theme,
            })}
            {ThemeButton(ThemeButtonProps {
                theme_name: "aqua",
                set_theme,
            })}
        </ul>
    }
}

#[component]
pub fn ThemeButton(
    theme_name: &'static str,
    set_theme: WriteSignal<Option<String>>,
) -> impl IntoView {
    view! {
        <li>
            <label
                data-theme=theme_name
                class="btn w-full transition duration-200 hover:scale-105"
                on:click=move |_| {
                    let window = web_sys::window().unwrap();
                    let document = window.document().unwrap();
                    let html = document.document_element().unwrap();
                    html.set_attribute("data-theme", theme_name).unwrap();
                    set_theme.set(Some(theme_name.to_string()));
                }
            >
                {theme_name}
            </label>
        </li>
    }
}

#[component]
fn RedirectToSetupDefault() -> impl IntoView {
    let navigate = use_navigate();

    // redirect to setup/2
    Effect::new(move |_| {
        navigate("/setup/2", Default::default());
    });

    // than call setup
    SetUp()
}

#[derive(Params, PartialEq, Clone, Debug)]
pub struct ParamNumPlayers {
    // Params isn't implemented for usize, only Option<usize>
    pub num: Option<usize>,
}

/// Renders the home page of your application.
#[component]
fn SetUp() -> impl IntoView {
    // get number of players from url
    let params = use_params::<ParamNumPlayers>();

    let setup_data = RwSignal::new(None::<SetupData>);

    let navigate = use_navigate();

    let parse_params = move |_| match params.get() {
        Err(_) => {
            navigate("/not-found", Default::default());
        }
        Ok(np) => match np.num {
            Some(num) => {
                if !(2..=4).contains(&num) {
                    navigate("/setup/2", Default::default());
                } else {
                    let np = NumPlayers::from(np);
                    setup_data.set(Some(SetupData::setup(np)));
                }
            }
            None => {
                navigate("/setup/2", Default::default());
            }
        },
    };

    Effect::new(parse_params);

    let new_setup = move |_| {
        if let Some(setup) = setup_data.get() {
            let current_num_players = setup.num_players;
            setup_data.set(Some(SetupData::setup(current_num_players)));
        }
    };

    view! {
        <Show when=move || setup_data.get().is_some() fallback=|| view! { <p>"Lade Setup..."</p> }>
            {move || {
                let setup = setup_data.get().unwrap();
                view! {
                    <h2 class="text-2xl text-center font-semibold tracking-wide text-secondary mb-4">
                        "Setup für "
                        <strong>{setup.num_players.to_string().to_lowercase()}</strong>" Spieler"
                    </h2>

                    <div class="amulet-grid">
                        <For
                            each=move || setup.removals.clone()
                            key=|removal| (removal.amulet_type, removal.count)
                            children=move |removal| {
                                view! {
                                    <div class="amulet-cell">
                                        <p class="text-xl pl-6 pr-3 font-semibold text-base-content">
                                            <strong>{removal.count}</strong>
                                            "x"
                                        </p>
                                        <img
                                            class="max-w-7/10 object-contain"
                                            src=removal.amulet_type.image_path()
                                            alt=removal.amulet_type.alt_text()
                                        />
                                    </div>
                                }
                            }
                        />
                    </div>

                    <button class="btn text-2xl mt-6 mx-auto block" on:click=new_setup>
                        "Neues Setup"
                    </button>
                }
            }}
        </Show>
    }
}
