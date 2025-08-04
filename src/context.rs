// provide context for application
// using a central context file to prevent type collisions in context

use crate::amulets::SetupData;
use leptos::prelude::*;

pub struct InputIdSetup(pub bool);

pub fn provide_app_context() {
    // context of setup data
    let setup_data = RwSignal::new(None::<SetupData>);
    provide_context(setup_data);

    // context of input id setup
    let input_id_setup = RwSignal::new(InputIdSetup(false));
    provide_context(input_id_setup);

    // context of theme name
    // beware that there is no type protection, if we do not wrap the signal in a user defined type!
    let (theme, set_theme) = signal(None::<String>);
    provide_context(set_theme);
    provide_context(theme);

    // use effect to set the theme from local storage or default to 'aqua'
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
