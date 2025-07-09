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
        <html lang="de" data-theme="dark">
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
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        // <Stylesheet id="leptos" href="/pkg/dh-amulets.css"/>
        // <Stylesheet href="/pkg/dh-amulets.css" />

        // sets the document title
        <Title text="Drachenhüter Amulett Setup" />

        // content for this welcome page
        <Router>
            <h1>"Drachenhüter Amulett Setup"</h1>
            <nav class="navbar bg-primary text-primary-content">
                <A href="/">"Home"</A>
                <A href="/setup/2">"Two Player"</A>
                <A href="/setup/3">"Three Player"</A>
                <A href="/setup/4">"Four Player"</A>
            </nav>
            <main>
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

/// Renders not found view
#[component]
fn NotFoundView() -> impl IntoView {
    "Page not found."
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let set_theme = |theme: &str| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html = document.document_element().unwrap();
        html.set_attribute("data-theme", theme).unwrap();
    };

    view! {
        <h2>"Willkommen!"</h2>
        <p>
            "Dies ist ein nichtkommerzielles Fanprojekt für das Setup von "
            <a href="https://www.kosmos.de/de/drachenhuter_1683757_4002051683757">
                "Drachenhüter"
            </a>", einem Kartenspiel
            erschienen im "<a href="https://www.kosmos.de">"Kosmosverlag"</a>.
        </p>
        <p>"Wähle im Navigationsmenü die Anzahl der Spieler aus."</p>
        <p>
            "Es wird eine zufällige Kombination an Amuletten berechnet, die aus dem Vorrat entfernt werden müssen."
        </p>
        <p>"Jeder Reload führt zu einem neuen Setup."</p>
        <p>"Fertig."</p>
        <p>"Viel Spaß beim Spielen 😊"</p>
        <div class="dropdown">
            <div tabindex="0" role="button" class="btn m-1">
                Theme wählen
            </div>
            <ul
                tabindex="0"
                class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52"
            >
                <li>
                    <label
                        data-theme="light"
                        class="btn w-full transition duration-200 hover:scale-105"
                        on:click=move |_| set_theme("light")
                    >
                        Light
                    </label>
                </li>
                <li>
                    <label
                        data-theme="dark"
                        class="btn w-full transition duration-200 hover:scale-105"
                        on:click=move |_| set_theme("dark")
                    >
                        Dark
                    </label>
                </li>
                <li>
                    <label
                        data-theme="cupcake"
                        class="btn w-full transition duration-200 hover:scale-105"
                        on:click=move |_| set_theme("cupcake")
                    >
                        Cupcake
                    </label>
                </li>
                <li>
                    <label
                        data-theme="cyberpunk"
                        class="btn w-full transition duration-200 hover:scale-105"
                        on:click=move |_| set_theme("cyberpunk")
                    >
                        Cyberpunk
                    </label>
                </li>
            </ul>
        </div>
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
                if num < 2 || num > 4 {
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
                    <h2>{format!("Setup für {} Spieler", setup.num_players)}</h2>
                    <button class="btn" on:click=new_setup>
                        "Neues Setup"
                    </button>

                    <div class="amulet-grid">
                        <For
                            each=move || setup.removals.clone()
                            key=|removal| (removal.amulet_type.clone(), removal.count)
                            children=move |removal| {
                                view! {
                                    <div class="amulet-cell">
                                        <img
                                            src=removal.amulet_type.image_path()
                                            alt=removal.amulet_type.alt_text()
                                        />
                                        <p>{removal.count}</p>
                                    </div>
                                }
                            }
                        />
                    </div>
                }
            }}
        </Show>
    }
}
