mod api;

use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/ressources")]
    Ressources {},
    #[route("/ressource/:id")]
    RessourceDetails { id: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}


/// Home page
#[component]
fn Home() -> Element {
    let details = use_resource(api::functions::get_game_infos);
    rsx! {
        div {}

    }
}

/// Ressources page
#[component]
fn Ressources() -> Element {
    rsx! {
        div {}

    }
}

/// RessourceDetails page
#[component]
fn RessourceDetails(id: String) -> Element {
    rsx! {
        div {}

    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Ressources { },
                "Ressources"
            }
            // Link {
            //     to: Route::Blog { id: 1 },
            //     "Blog"
            // }
        }

        Outlet::<Route> {}
    }
}
