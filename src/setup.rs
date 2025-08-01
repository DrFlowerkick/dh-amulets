// setup route of app

use crate::amulets::{NumPlayers, SetupData, SetupId};
use leptos::{
    ev::{Event, KeyboardEvent},
    prelude::*,
};
use leptos_router::{
    hooks::{use_navigate, use_params},
    params::Params,
};
use web_sys::window;

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

    let input_id_setup =
        use_context::<RwSignal<InputIdSetup>>().expect("InputIdSetup context not found");

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
                    if input_id_setup.read_untracked().0 {
                        // if input_id_setup is true, we do not set the setup_data
                        // because it is set by the SetUpId component
                        input_id_setup.set(InputIdSetup(false));
                    } else {
                        let np = NumPlayers::from(np);
                        setup_data.set(Some(SetupData::setup(np)));
                    }
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
    let (valid_id, set_valid_id) = signal(true);
    let (show_copied_toast, set_show_copied_toast) = signal(false);

    let setup_data =
        use_context::<RwSignal<Option<SetupData>>>().expect("SetupData context not found");

    let input_id_setup =
        use_context::<RwSignal<InputIdSetup>>().expect("InputIdSetup context not found");

    Effect::new(move || {
        if let Some(setup_data) = setup_data.get() {
            if let Some(id) = SetupId::encode(&setup_data) {
                set_setup_id.set(Some(id.to_hex_string()));
                set_valid_id.set(true);
            }
        }
    });

    let copy_to_clipboard = move || {
        if let Some(id) = setup_id.get() {
            let clipboard = window()
                .expect("should have a Window")
                .navigator()
                .clipboard();

            let _ = clipboard.write_text(&id);

            // show toast
            set_show_copied_toast.set(true);
            // Hide after 2 seconds
            set_timeout(
                move || {
                    set_show_copied_toast.set(false);
                },
                std::time::Duration::from_secs(2),
            );
        }
    };

    let check_input = move |ev: Event| {
        let input = event_target_value(&ev);
        if input == "NoSetup" {
            set_valid_id.set(true);
            return;
        }
        if let Some(hex_is_valid) = SetupId::from_hex_string(&input) {
            let is_valid = hex_is_valid.decode().is_some();
            set_valid_id.set(is_valid);
            set_setup_id.set(Some(input));
        } else {
            set_valid_id.set(false);
        }
    };

    let reset_id = move || {
        // reset input to previous valid ID
        if let Some(previous_valid_id) = setup_id.get() {
            // signal old value as new value
            set_setup_id.set(Some(previous_valid_id));
        } else {
            set_setup_id.set(None);
        }
    };

    let apply_id = move |id: &str| {
        let setup = SetupId::from_hex_string(id)
            .expect("Expecting valid id")
            .decode()
            .expect("Expecting valid setup data");
        let route = match setup.num_players {
            NumPlayers::Two => "/setup/2",
            NumPlayers::Three => "/setup/3",
            NumPlayers::Four => "/setup/4",
        };
        setup_data.set(Some(setup));
        input_id_setup.set(InputIdSetup(true));
        let navigate = use_navigate();
        navigate(route, Default::default());
    };

    let input_ref: NodeRef<leptos::html::Input> = NodeRef::new();

    let check_keydown = move |ev: KeyboardEvent| {
        match ev.key().as_str() {
            "Enter" => {
                if valid_id.get() {
                    // accept entry
                    //let id = event_target_value(&ev);
                    //apply_id(&id);
                    if let Some(input) = input_ref.get() {
                        input.blur().unwrap_or_default(); // blur input on Enter
                    }
                } else {
                    // reject entry -> prevent_default()
                    ev.prevent_default();
                }
            }
            "Escape" => {
                reset_id();
            }
            _ => {}
        }
    };

    let check_blur = move |_| {
        // on blur, if valid, apply the ID
        if valid_id.get() {
            if let Some(id) = setup_id.get() {
                apply_id(&id);
            }
        } else {
            // reset to previous valid ID
            reset_id();
        }
    };

    view! {
        <div class="flex items-center gap-2 text-base font-semibold">
            <label for="setup-id-input" class="whitespace-nowrap">
                "Setup ID:"
            </label>

            <input
                id="setup-id-input"
                data-testid="setup-id"
                aria-label="Setup ID Input"
                class="font-mono bg-transparent border-none focus:outline-none w-auto min-w-0 max-w-[7ch] truncate transition-colors duration-200"
                class:text-primary=move || valid_id.get()
                class:text-red-500=move || !valid_id.get()
                value=move || setup_id.get().unwrap_or("NoSetup".to_string())
                on:input=check_input
                on:keydown=check_keydown
                on:blur=check_blur
                node_ref=input_ref
            />

            <div class="relative">
                <button
                    on:click=move |_| copy_to_clipboard()
                    class="text-base leading-none transition duration-200 cursor-pointer"
                    aria-label="Copy to clipboard"
                >
                    <span
                        class="inline-block relative bottom-[5px]"
                        class:animate-bounce=move || show_copied_toast.get()
                        class:animate-none=move || !show_copied_toast.get()
                        data-testid="clipboard"
                    >
                        "📋"
                    </span>
                </button>

                <div
                    class="absolute -top-7 left-1/2 -translate-x-1/2 z-50
                    text-base px-3 py-1 rounded shadow 
                    bg-[color:var(--b1)/50%] backdrop-blur-sm
                    transition-opacity duration-1000 ease-in-out
                    pointer-events-none select-none"
                    class:opacity-100=move || show_copied_toast.get()
                    class:opacity-0=move || !show_copied_toast.get()
                    data-testid="copy-toast"
                >
                    "✔Kopiert"
                </div>
            </div>
        </div>
    }
}

pub struct InputIdSetup(pub bool);

pub fn provide_setup_context() {
    let setup_data = RwSignal::new(None::<SetupData>);
    provide_context(setup_data);

    let input_id_setup = RwSignal::new(InputIdSetup(false));
    provide_context(input_id_setup);
}