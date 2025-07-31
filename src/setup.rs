// setup route of app

use crate::amulets::{NumPlayers, SetupData, SetupId};
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

#[component]
pub fn SetUp() -> impl IntoView {
    // get number of players from url
    let params = use_params::<ParamNumPlayers>();

    let setup_data =
        use_context::<RwSignal<Option<SetupData>>>().expect("SetupData context not found");

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
                    <button
                        data-testid="setup-heading"
                        aria-label="Neues Setup"
                        title="Klicke für ein neues Setup"
                        on:click=new_setup
                        class="btn btn-ghost text-2xl font-semibold tracking-wide text-secondary mb-4 flex items-center justify-center gap-2 mx-auto"
                    >
                        <p>
                            "Setup für "
                            <strong>{setup.num_players.to_string().to_lowercase()}</strong>
                            " Spieler"
                        </p>
                        // Reload Icon
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            class="w-6 h-6"
                        >
                            <path d="M2 12a10 10 0 0 1 18.4-5.6" />
                            <polyline points="20 1 20.4 6.4 15 6" />
                            <path d="M22 12a10 10 0 0 1-18.4 5.6" />
                            <polyline points="4 23 3.6 17.6 9 18" />
                        </svg>
                    </button>

                    <div class="amulet-grid">
                        <For
                            each=move || setup.removals.clone()
                            key=|removal| (removal.amulet_type, removal.count)
                            children=move |removal| {
                                view! {
                                    <div class="amulet-cell">
                                        <p class="text-xl pr-2 font-semibold text-base-content">
                                            <strong>{removal.count}</strong>
                                            "x"
                                        </p>
                                        <img
                                            class="max-w-8/10 object-contain"
                                            src=removal.amulet_type.image_path()
                                            alt=removal.amulet_type.alt_text()
                                        />
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

#[component]
pub fn SetUpId() -> impl IntoView {
    let (setup_id, set_setup_id) = signal(None::<String>);

    let setup_data =
        use_context::<RwSignal<Option<SetupData>>>().expect("SetupData context not found");

    Effect::new(move || {
        if let Some(setup_data) = setup_data.get() {
            if let Some(id) = SetupId::encode(&setup_data) {
                set_setup_id.set(Some(id.to_hex_string()));
            }
        }
    });

    view! {
        <div class="setup-id-container">
            <p class="text-base font-semibold mb-1">
                "Setup ID: "<span class="text-primary" data-testid="setup-id">
                    {move || setup_id.get().unwrap_or("No Setup ID".to_string())}
                </span>
            </p>
        </div>
    }
}
