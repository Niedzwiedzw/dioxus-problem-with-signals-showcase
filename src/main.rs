#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Debug, Clone)]
struct Something(i32);

impl PartialEq for Something {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

#[component]
fn LevelTwo(count_1: Something, count_2: ReadOnlySignal<Something>) -> Element {
    rsx! {
        div {
            class: "LevelThree",
            // "count_1: {count_1}"
            // "count_2: {count_2}"
        }
    }
}

#[component]
fn LevelOne(count_1: ReadOnlySignal<Something>, count_2: ReadOnlySignal<Something>) -> Element {
    rsx! {
        LevelTwo {
            count_1: count_2.read().clone(),
            count_2
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count_1 = use_signal(|| Something(2137));
    let mut count_2 = use_signal(|| Something(2138));
    rsx! {
        div {
            h1 { "High-Five counter" }
            LevelOne {
                count_1: ReadOnlySignal::from(count_1),
                count_2: ReadOnlySignal::from(count_2)
            }
        }
    }
}
