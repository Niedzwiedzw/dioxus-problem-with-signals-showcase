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

#[derive(Debug, Clone, PartialEq)]
struct Something(i32);

mod event_handler {
    use std::{cell::RefCell, rc::Rc};

    impl<F: FnMut(T) + 'static, T: 'static> From<F> for EventHandler<T> {
        fn from(callback: F) -> Self {
            EventHandler::new(callback)
        }
    }

    #[derive(Clone)]
    struct EventHandler<T> {
        callback: Rc<RefCell<dyn FnMut(T)>>,
    }

    impl<T> EventHandler<T>
    where
        T: 'static,
    {
        fn new<F>(callback: F) -> Self
        where
            F: FnMut(T) + 'static,
        {
            Self {
                callback: Rc::new(RefCell::new(callback)),
            }
        }

        fn call(&self, value: T) {
            self.callback
                .try_borrow_mut()
                .map(|mut callback| callback(value))
                .expect("event handlers to be always callable");
        }

        fn map<U, F>(self, mut map: F) -> EventHandler<U>
        where
            U: 'static,
            F: FnMut(U) -> T + 'static,
        {
            (move |value| self.call(map(value))).into()
        }
    }
}

#[component]
fn LevelTwo(
    count_1: Something,
    count_2: MappedSignal<Something>,
    depth: usize,
    handle: EventHandler<()>,
) -> Element {
    rsx! {
        Container {
           div {
               Container {
                   if count_1.0 % 2 == 0 {
                        div {
                            class: "LevelThree",
                            onclick: move |_| {
                                handle.call(());
                            },
                            "count_1: {count_1:?}",
                            "count_2: {count_2:?}"
                        }
                   }
               }

                Container {
                   if count_1.0 % 3 == 0 {
                        div {
                            class: "LevelThree",
                            onclick: move |_| {
                                handle.call(());
                            },
                            "count_1: {count_1:?}",
                            "count_2: {count_2:?}"
                        }
                   }
                }
            }
        }
    }
}

#[component]
fn LevelOne(
    count_1: MappedSignal<Something>,
    count_2: MappedSignal<Something>,
    children: Element,
    handle: EventHandler<()>,
) -> Element {
    rsx! {
        Container {
            for _ in 0..10000 {
                LevelTwo {
                    count_1: count_1.read().clone(),
                    count_2: count_2.clone(),
                    depth: 15,
                    handle: move |ev|handle.call(ev),
                }
            }
        }
    }
}

#[component]
fn Container(children: Element) -> Element {
    rsx! {
        div {
            class: "Container",
            { children }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count_1 = use_signal(|| Something(1));
    let mut count_2 = use_signal(|| Something(2138));
    let mut show = use_signal(|| false);
    rsx! {
        div {
            h1 { "High-Five counter" }

            button { onclick: move |_| {
                let old = *show.read();
                *show.write() = !old;
            }, "show/hide" }
            Container {
                if *show.read() {
                    Container {
                        button { onclick: move |_| {count_1.write().0 += 1}, "bump count_1" }
                        button { onclick: move |_| {count_2.write().0 += 1}, "bump count_2" }
                        LevelOne {
                            count_1: count_1.map(|v| v),
                            count_2: count_2.map(|v| v),
                            handle: |_| {}
                        }
                    }
                }
            }
        }
    }
}
