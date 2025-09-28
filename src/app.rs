use cross_clipboard::copy_to_clipboard;

use cross_storage::storage_get;
use dioxus::prelude::*;

use dioxus_tw_components::prelude::DioxusTwComponentsBootstrap;
use lucide_dioxus::{Copy, Pencil, Trash2};
use wingedcap::client::{ServerMeta, ServerWithMeta};

use crate::{
    components::{EditMetaDialog, KeyTableComponent},
    storage::{del_stored_key, get_stored_keys},
    ui::{
        button::{Button, ButtonVariant},
        popover::POPOVER_TARGET_ID,
    },
    utils::wait,
};

const FAVICON: Asset = asset!("/src/theme/fav.svg");

const CSS_HREF: Asset = asset!("/tmp/styles.css");
const CSS_STYLE: &str = include_str!("../tmp/styles.css");

#[derive(Props, PartialEq, Clone)]
pub struct AppProps {
    pub tunnel_host: String,
    pub server_id: String,
    pub server_pk: String,
}

#[component]
pub fn App() -> Element {
    let AppProps {
        tunnel_host,
        server_id,
        server_pk,
    } = use_context::<AppProps>().clone();

    let mut is_edit_meta_dialog_open = use_signal(|| false);

    let mut key_records = use_signal(|| Vec::new());

    let mut refresh_key_records = move || {
        let current_key_records = get_stored_keys().unwrap_or_default();

        key_records.set(current_key_records);
    };

    let delete_all_key_records = move || {
        for key_record in key_records() {
            let _ = del_stored_key(key_record.service);
        }
    };

    // Auto-refresh key records every second
    use_future(move || async move {
        loop {
            refresh_key_records();
            wait(1000).await;
        }
    });

    let on_copy_server_data = move |_| {
        let server_host = format!("{tunnel_host}/{server_id}");

        let meta = match storage_get("server_meta") {
            Ok(meta) => match serde_json::from_str(&meta) {
                Ok(meta) => meta,
                Err(_) => ServerMeta::default(),
            },
            Err(_) => ServerMeta::default(),
        };

        let server_with_meta = ServerWithMeta {
            host: server_host,
            pk: server_pk.clone(),
            meta: Some(meta),
        };

        let server_json = serde_json::to_string_pretty(&server_with_meta).unwrap();

        let _ = copy_to_clipboard(&server_json);
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }

        document::Link { rel: "stylesheet", href: CSS_HREF }
        style { {CSS_STYLE} }

        // launches Dioxus Tailwind Components
        // some components may not work without this
        DioxusTwComponentsBootstrap {}

        // required for popover components
        div { id: POPOVER_TARGET_ID }




        main { class: "relative mx-auto max-w-200 h-screen w-screen min-w-90 grow px-4",
            div { class: "relative flex h-full items-center justify-center",
                div { class: "w-full self-start pt-[20vh]",
                    div { class: "flex items-center gap-2 mb-6",
                        Button {
                            class: "w-min gap-2",
                            onclick: on_copy_server_data,

                            "Copy"
                            Copy { class: "size-4" }
                        }

                        Button {
                            variant: ButtonVariant::Outline,
                            class: "w-min gap-2",
                            onclick: move |_| is_edit_meta_dialog_open.set(true),

                            "Edit"
                            Pencil { class: "size-5" }
                        }

                        EditMetaDialog {
                            open: is_edit_meta_dialog_open(),
                            onopenchange: move |_open| is_edit_meta_dialog_open.set(_open),
                        }

                        Button {
                            variant: ButtonVariant::Destructive,
                            class: "w-min gap-2 ml-auto",
                            onclick: move |_| delete_all_key_records(),

                            "Delete"
                            Trash2 { class: "size-4" }
                        }
                    }

                    KeyTableComponent { key_records: key_records() }
                }
            }
        }

    }
}
