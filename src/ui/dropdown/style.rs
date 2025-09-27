use super::props::*;
use dioxus::prelude::*;
use dioxus_tw_components::attributes::*;

impl Class for DropdownProps {
    fn base(&self) -> &'static str {
        "z-20 relative text-foreground"
    }
}

impl Class for DropdownToggleProps {
    fn base(&self) -> &'static str {
        "text-sm font-medium bg-background rounded-md whitespace-nowrap hover:bg-accent hover:text-accent-foreground"
    }
}

impl Class for DropdownContentProps {
    fn base(&self) -> &'static str {
        "z-20 p-2 space-y-2 min-w-full bg-background flex flex-col rounded-md border border-input shadow absolute mt-2 left-1/2 -translate-x-1/2 whitespace-nowrap opacity-100 data-[state=inactive]:invisible"
    }

    fn animation(&self) -> Option<&'static str> {
        Some(match *self.animation.read() {
            Animation::None => "transition-none",
            Animation::Light | Animation::Full => {
                "transition-all duration-100 data-[state=inactive]:scale-90 data-[state=active]:scale-100 data-[state=inactive]:opacity-0"
            }
        })
    }
}
