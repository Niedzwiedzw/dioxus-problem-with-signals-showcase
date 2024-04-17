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

#[derive(Props, PartialEq, Clone)]
struct LevelFourProps {
    count_1: Something,
    count_2: ReadOnlySignal<Something>,
}

fn LevelFour(LevelFourProps { count_1, count_2 }: LevelFourProps) -> Element {
    rsx! {
        div {
            class: "LevelFour",
            // "count_1: {count_1}"
            // "count_2: {count_2}"
        }
    }
}
#[derive(Props, PartialEq, Clone)]
struct LevelThreeProps {
    count_1: Something,
    count_2: ReadOnlySignal<Something>,
}

fn LevelThree(LevelThreeProps { count_1, count_2 }: LevelThreeProps) -> Element {
    rsx! {
        div {
            class: "LevelThree",
            // "count_1: {count_1}"
            // "count_2: {count_2}"
            LevelFour {
                count_1,
                count_2
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct LevelTwoProps {
    count_1: Something,
    count_2: ReadOnlySignal<Something>,
}

fn LevelTwo(LevelTwoProps { count_1, count_2 }: LevelTwoProps) -> Element {
    rsx! {
        LevelThree {
            count_1,
            count_2
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct LevelOneProps {
    count_1: ReadOnlySignal<Something>,
    count_2: ReadOnlySignal<Something>,
}

fn LevelOne(LevelOneProps { count_1, count_2 }: LevelOneProps) -> Element {
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
            button { onclick: move |_| count_1.write().0 += 0, "bump count_1" }
            button { onclick: move |_| count_2.write().0 += 0, "bump count_2" }
            LevelOne {
                count_1: ReadOnlySignal::from(count_1),
                count_2: ReadOnlySignal::from(count_2)
            }
        }
    }
}
