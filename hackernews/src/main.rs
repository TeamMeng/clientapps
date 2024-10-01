use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use hackernews::App;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    let path = route.join("/");
    rsx!(
        div {
            class: "flex flex-col items-center justify-center h-screen",
            h1 { class: "text-3xl font-bold mb-4 p-4", "Not Found: The page `/{path}` you requested is missing" }
        }
    )
}
