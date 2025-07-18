// theme management of app

use leptos::prelude::*;

#[component]
pub fn ThemeSelector() -> impl IntoView {
    let theme_display = use_context::<ReadSignal<Option<String>>>().expect("theme_display missing");

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
        <button
            class="btn mx-auto md:mx-0 block"
            popovertarget="popover-1"
            style="anchor-name:--anchor-1"
        >
            "Thema wählen (Aktuell: "
            {move || theme_display.get()}
            ")"
        </button>
        <ul
            class="dropdown dropdown-top menu w-52 rounded-box bg-base-100 shadow-sm"
            popover
            id="popover-1"
            style="position-anchor:--anchor-1"
        >
            {ThemeButton(ThemeButtonProps {
                theme_name: "fantasy",
            })}
            {ThemeButton(ThemeButtonProps {
                theme_name: "caramellatte",
            })}
            {ThemeButton(ThemeButtonProps {
                theme_name: "coffee",
            })}
            {ThemeButton(ThemeButtonProps {
                theme_name: "business",
            })}
            {ThemeButton(ThemeButtonProps {
                theme_name: "synthwave",
            })}
            {ThemeButton(ThemeButtonProps {
                theme_name: "aqua",
            })}
        </ul>
    }
}

#[component]
pub fn ThemeButton(theme_name: &'static str) -> impl IntoView {
    let theme_context =
        use_context::<WriteSignal<Option<String>>>().expect("ThemeContext not found");

    view! {
        <li>
            <label
                data-theme=theme_name
                class="btn w-full transition duration-200 hover:scale-105"
                on:click=move |_| {
                    if let Some(window) = web_sys::window() {
                        if let Some(html) = window.document().and_then(|doc| doc.document_element())
                        {
                            html.set_attribute("data-theme", theme_name).unwrap();
                        }
                        if let Ok(Some(local_storage)) = window.local_storage() {
                            local_storage.set_item("data-theme", theme_name).unwrap();
                        }
                    }
                    theme_context.set(Some(theme_name.to_string()));
                }
            >
                {theme_name}
            </label>
        </li>
    }
}

pub fn provide_theme_context() {
    let (theme, set_theme) = signal(None::<String>);
    // beware that there is no type protection, if we do not wrap the signal in a user defined type!
    provide_context(set_theme);
    provide_context(theme);

    Effect::new(move |_| {
        if theme.get().is_none() {
            if let Some(window) = web_sys::window() {
                // theme is set in script in head to local_storage value or aqua as default value.
                // this prevents flickering.
                // here we only load theme from dom.
                if let Some(theme_name) = window
                    .document()
                    .and_then(|doc| doc.document_element())
                    .and_then(|el| el.get_attribute("data-theme"))
                {
                    set_theme.set(Some(theme_name));
                }
            }
        }
    });
}
