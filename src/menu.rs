// menu and menu button in header

use crate::themes::ThemeSelector;
use leptos::prelude::*;

#[component]
pub fn MenuButton() -> impl IntoView {
    view! {
        <div class="relative">
            // Hamburger button
            <button
                aria-label="Menü"
                class="btn btn-ghost"
                popovertarget="popover-menu"
                style="anchor-name:--anchor-menu"
            >
                <div class="space-y-1">
                    <div class="w-6 h-[2px] bg-primary"></div>
                    <div class="w-6 h-[2px] bg-primary"></div>
                    <div class="w-6 h-[2px] bg-primary"></div>
                </div>
            </button>

            <ul
                id="popover-menu"
                popover
                class="dropdown dropdown-end mt-2 menu rounded-box bg-base-100 shadow-md p-2 w-56"
                style="position-anchor:--anchor-menu"
            >
                <h3 class="text-lg font-semibold mb-1">"Menü"</h3>
                <hr role="separator" class="my-2 border-t border-primary" />
                <ThemeSelector />
            </ul>
        </div>
    }
}
