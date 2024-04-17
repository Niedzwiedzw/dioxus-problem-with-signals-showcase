#![allow(non_snake_case)]

use std::convert::identity;

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
fn LevelTwo(count_1: Something, count_2: MappedSignal<Something>) -> Element {
    rsx! {
        div {
            class: "LevelThree",
            // "count_1: {count_1}"
            // "count_2: {count_2}"
        }
    }
}

#[component]
fn LevelOne(count_1: MappedSignal<Something>, count_2: MappedSignal<Something>) -> Element {
    rsx! {
        LevelTwo {
            count_1: count_1.read().clone(),
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
                count_1: count_1.map(|v| v),
                count_2: count_2.map(|v| v),
            }
        }
    }
}
