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

#[derive(PartialEq, Clone, Copy)]
struct Readonly<T: 'static> {
    signal: Signal<T>,
}

impl<T> From<Signal<T>> for Readonly<T> {
    fn from(signal: Signal<T>) -> Self {
        Readonly { signal }
    }
}

impl PartialEq for Something {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

#[component]
fn LevelTwo(count_1: Something, count_2: Readonly<Something>) -> Element {
    rsx! {
        div {
            class: "LevelThree",
            // "count_1: {count_1}"
            // "count_2: {count_2}"
        }
    }
}

#[component]
fn LevelOne(count_1: Readonly<Something>, count_2: Readonly<Something>) -> Element {
    rsx! {
        LevelTwo {
            count_1: {
                let Something(count_1) = count_1.signal.read().clone();
                Something(count_1 + 1)
            },
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
            button { onclick: move |_| {count_1.write().0 += 1}, "bump count_1" }
            button { onclick: move |_| {count_2.write().0 += 1}, "bump count_2" }
            LevelOne {
                count_1: Readonly::from(count_1),
                count_2: Readonly::from(count_2)
            }
        }
    }
}
