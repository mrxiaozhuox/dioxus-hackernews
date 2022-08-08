#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_toast::{ToastFrame, ToastManager};

mod components;
mod pages;
mod api;

use crate::{components::nav::Navbar, pages::news::NewsPage};

static TOAST_MANAGER: dioxus::fermi::AtomRef<ToastManager> = |_| ToastManager::default();

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Powered by Dioxus Starter: https://github.com/mrxiaozhuox/dioxus-starter");
    dioxus::web::launch(App)
}

fn App(cx: Scope) -> Element {
    // init mode information
    cx.render(rsx! {
        // dioxus toast manager init
        ToastFrame {
            manager: use_atom_ref(&cx, TOAST_MANAGER),
            maximum: 6,
        }
        // dioxus router info
        Router {
            Navbar {}
            div {
                class: "container mx-auto",
                Route {
                    to: "/",
                    Redirect { to: "/news/1" }
                }
                Route {
                    to: "/news/:page",
                    NewsPage {}
                }
                Route {
                    to: "",
                    pages::_404::NotFound {}
                }
            }
        }
    })
}
