// central app definitions
use crate::home::HomePage;
use crate::setup::SetUp;
use crate::themes::provide_theme_context;
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{A, Route, Router, Routes},
    hooks::use_navigate,
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="de">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <Stylesheet id="leptos" href="/pkg/dh-amulets.css" />
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
                <link rel="manifest" href="/manifest.json" />
                // use script to:
                // 1.) set theme from local_storage or set to aqua as default, if no local storage.
                // 2.) registration of the service worker for PWA.
                <script>
                    r#"
                        const theme = localStorage.getItem('data-theme') || 'aqua';
                        document.documentElement.setAttribute('data-theme', theme);

                        if ('serviceWorker' in navigator) {
                            window.addEventListener('load', function () {
                                navigator.serviceWorker.register('/sw.js')
                                    .then(function (registration) {
                                        console.log('ServiceWorker registration successful:', registration);
                                    })
                                    .catch(function (err) {
                                        console.log('ServiceWorker registration failed:', err);
                                    });
                            });
                        }
                    "#
                </script>
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

    // provide theme context
    provide_theme_context();

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

/// Renders not found view
#[component]
fn NotFoundView() -> impl IntoView {
    "Page not found."
}
