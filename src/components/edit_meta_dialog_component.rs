use dioxus::prelude::*;
use lucide_dioxus::{Info, Save};

use crate::{
    storage::{get_stored_meta, store_meta},
    ui::{
        button::{Button, ButtonVariant},
        card::{CardContent, CardDescription, CardHeader, CardTitle},
        hovercard::{HoverCard, HoverCardContent, HoverCardTrigger},
        input_animated_label::InputAnimatedLabel,
        modal::{Modal, ModalBackground, ModalContent},
    },
};

#[derive(Props, PartialEq, Clone)]
pub struct EditMetaDialogProps {
    #[props(optional)]
    open: bool,
    #[props]
    onopenchange: Callback<bool>,
}

#[component]
pub fn EditMetaDialog(props: EditMetaDialogProps) -> Element {
    let mut form_data = use_signal(|| get_stored_meta());

    let handle_submit = move || {
        let _ = store_meta(form_data.read().clone());
        props.onopenchange.call(false);
    };

    rsx! {
        Modal { open: props.open, onopenchange: props.onopenchange,
            ModalBackground {}

            ModalContent { class: "",
                CardHeader { class: "mb-6",
                    CardTitle { "Edit Metadata" }

                    CardDescription { "Help clients identify your server" }
                }

                CardContent { class: "w-full",
                    form {
                        class: "w-full space-y-3",
                        onsubmit: move |_| handle_submit(),
                        div { class: "flex items-center gap-2 w-full",
                            InputAnimatedLabel {
                                label: "Provider",
                                value: form_data.read().provider.clone(),
                                oninput: move |e: FormEvent| {
                                    form_data
                                        .with_mut(|data| {
                                            data.provider = {
                                                let value = e.value();
                                                if value == "" { None } else { Some(value) }
                                            };
                                        })
                                },
                                container_class: "grow",
                            }

                            HoverCard {
                                HoverCardTrigger {
                                    Button {
                                        r#type: "button",
                                        variant: ButtonVariant::Ghost,
                                        class: "px-0",
                                        Info { class: "text-blue-500" }
                                    }
                                }

                                HoverCardContent { class: "max-w-50", "Responsible entity" }
                            }
                        }

                        div { class: "flex items-center gap-2 w-full",
                            InputAnimatedLabel {
                                label: "Hoster",
                                value: form_data.read().hoster.clone(),
                                oninput: move |e: FormEvent| {
                                    form_data
                                        .with_mut(|data| {
                                            data.hoster = {
                                                let value = e.value();
                                                if value == "" { None } else { Some(value) }
                                            };
                                        })
                                },
                                container_class: "grow",
                            }

                            HoverCard {
                                HoverCardTrigger {
                                    Button {
                                        r#type: "button",
                                        variant: ButtonVariant::Ghost,
                                        class: "px-0",
                                        Info { class: "text-blue-500" }
                                    }
                                }

                                HoverCardContent { class: "max-w-50", "Hosting provider" }
                            }
                        }

                        div { class: "flex items-center gap-2 w-full",
                            InputAnimatedLabel {
                                label: "Location",
                                value: form_data.read().location.clone(),
                                oninput: move |e: FormEvent| {
                                    form_data
                                        .with_mut(|data| {
                                            data.location = {
                                                let value = e.value();
                                                if value == "" { None } else { Some(value) }
                                            };
                                        })
                                },
                                container_class: "grow",
                            }

                            HoverCard {
                                HoverCardTrigger {
                                    Button {
                                        r#type: "button",
                                        variant: ButtonVariant::Ghost,
                                        class: "px-0",
                                        Info { class: "text-blue-500" }
                                    }
                                }

                                HoverCardContent { class: "max-w-50", "Geographical location" }
                            }
                        }

                        div { class: "flex items-center justify-between gap-4 w-full mt-4",
                            Button { class: "flex items-center justify-between w-max gap-4",
                                "Save"
                                Save { class: " h-4 w-4 shrink-0" }
                            }
                        }
                    }
                }
            }
        }
    }
}
