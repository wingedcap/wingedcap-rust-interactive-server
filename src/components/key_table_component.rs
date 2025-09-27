use dioxus::prelude::*;

use dioxus_tw_components::prelude::{
    Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
};
use lucide_dioxus::{LockKeyhole, LockKeyholeOpen, Trash2};
use wingedcap::{get_current_unix_time, server::KeyRecord};

use crate::{
    storage::del_stored_key,
    ui::button::{Button, ButtonVariant},
    utils::wait,
};

#[derive(Props, PartialEq, Clone)]
pub struct KeyTableComponentProps {
    pub key_records: Vec<KeyRecord>,
}

#[component]
pub fn KeyTableComponent(
    KeyTableComponentProps { key_records }: KeyTableComponentProps,
) -> Element {
    let mut now = use_signal(|| get_current_unix_time() as u64);

    // Auto-refresh clock every second
    use_future(move || async move {
        loop {
            now.set(get_current_unix_time() as u64);
            wait(1000).await;
        }
    });

    let del_key_record = move |service_id: String| {
        let _ = del_stored_key(service_id);
    };

    rsx! {
        Table { class: "",
            TableHeader { class: "border-b *:first:rounded-tl-md *:last:rounded-tr-md",

                TableHead { class: "bg-green-500/20",
                    p { class: "w-10", "Sender" }
                }

                TableHead { class: "bg-blue-500/20",
                    p { class: "w-10", "Receiver" }
                }

                TableHead { class: "bg-purple-500/20",
                    p { class: "w-10", "Key" }
                }


                TableHead { class: "bg-red-500/20",
                    p { class: "w-10", "Status" }
                }
            }

            TableBody { class: "",
                for key_record in key_records.clone() {
                    TableRow { class: "h-10 last:*:first:rounded-bl-md last:*:last:rounded-br-md relative",

                        TableCell { class: "relative bg-green-500/10",
                            div { class: "absolute inset-y-0 flex items-center inset-x-2 overflow-clip",
                                "{key_record.sender}"
                            }
                        }

                        TableCell { class: "relative bg-blue-500/10 ",
                            div { class: "absolute inset-y-0 flex items-center inset-x-2 overflow-clip",
                                "{key_record.receiver}"
                            }
                        }

                        TableCell { class: "relative bg-purple-500/10",
                            div { class: "absolute inset-y-0 flex items-center inset-x-2 overflow-clip",
                                "{key_record.key}"
                            }
                        }

                        TableCell { class: "relative bg-red-500/10",
                            div { class: "absolute inset-y-0 flex items-center inset-x-2 overflow-clip",
                                if key_record.unlocks_at > now() {
                                    div { class: "flex items-center gap-2",
                                        LockKeyhole { class: "size-4 stroke-destructive" }
                                        span { class: "text-sm", "({key_record.unlocks_at - now()} s)" }
                                    }
                                } else {
                                    LockKeyholeOpen { class: "size-4 stroke-green-500" }
                                }

                                Button {
                                    variant: ButtonVariant::Ghost,
                                    class: "px-0 size-8 ml-auto",
                                    onclick: move |_| del_key_record(key_record.service.clone()),
                                    Trash2 { class: "size-4 stroke-destructive" }
                                }
                            }
                        }
                    }
                }

                if key_records.len() == 0 {
                    TableRow { class: "h-10 last:*:first:rounded-bl-md last:*:last:rounded-br-md",
                        TableCell { class: "relative bg-muted w-full", colspan: "4",
                            div { class: "absolute inset-y-0 flex items-center justify-center inset-x-2",
                                span { class: "text-sm", "No keys stored" }
                            }
                        }
                    }
                }
            }
        }
    }
}
