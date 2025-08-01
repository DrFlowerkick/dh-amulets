// theme management of app

use leptos::prelude::*;

#[component]
pub fn ThemeSelector() -> impl IntoView {
    let theme_display = use_context::<ReadSignal<Option<String>>>().expect("theme_display missing");

    view! {
        <h3 class="text-lg font-semibold mb-1">"App Thema"</h3>
        <div class="flex items-center gap-2">
            <p class="text-base font-semibold mb-1">
                "Aktuell: "<span class="text-primary" data-testid="current-theme">
                    {move || theme_display.get()}
                </span>
            </p>
            <button
                class="ml-2 text-base leading-none transition duration-200 cursor-pointer"
                popovertarget="popover-1"
                style="anchor-name:--anchor-1"
                aria-label="Thema wählen"
            >
                <span class="inline-block relative bottom-[3px]">"🎨"</span>
            </button>
            <ul
                class="dropdown menu w-52 rounded-box bg-base-20 shadow-sm"
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
        </div>
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
