// setup route of app

use crate::amulets::{NumPlayers, SetupData};
use leptos::prelude::*;
use leptos_router::{
    hooks::{use_navigate, use_params},
    params::Params,
};

#[derive(Params, PartialEq, Clone, Debug)]
pub struct ParamNumPlayers {
    // Params isn't implemented for usize, only Option<usize>
    pub num: Option<usize>,
}

/// Renders the home page of your application.
#[component]
pub fn SetUp() -> impl IntoView {
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
