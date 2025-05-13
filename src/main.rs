use leptos::prelude::*;
use leptos_router::components::Router;

use lusr::LUSR;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| {
        view! {
            <Router>
                <LUSR />
            </Router>
            <noscript>"This page contains webassembly and javascript content, please enable javascript in your browser."</noscript>
        }
    })
}
