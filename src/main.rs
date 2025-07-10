use dioxus::logger::tracing::info;
use dioxus::prelude::*;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS },
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");

    let skip = move |evt| {
        info!("skip button clicked");
    };
    let save = move |evt| {
        info!("save button clicked");
    };

    rsx! {
        div { id: "dogview", 
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            button { onclick: skip,  id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    }
}


// continue reading here -> https://dioxuslabs.com/learn/0.6/guide/state#global-state-with-context
