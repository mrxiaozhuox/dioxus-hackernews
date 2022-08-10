#![allow(non_snake_case)]

use api::get_list;
use dioxus::prelude::*;
use dioxus_toast::{ToastFrame, ToastManager};

mod api;
mod components;
mod pages;

use crate::{components::nav::Navbar, pages::news::ListShow};

static TOAST_MANAGER: dioxus::fermi::AtomRef<ToastManager> = |_| ToastManager::default();

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Powered by Dioxus Starter: https://github.com/mrxiaozhuox/dioxus-starter");
    dioxus::web::launch(App)
}

#[derive(Debug, Clone)]
pub struct LoadedInfo {
    pub topstories: Vec<u64>,
    pub askstories: Vec<u64>,
    pub showstories: Vec<u64>,
    pub jobstories: Vec<u64>,
}

impl LoadedInfo {
    pub fn from_str(&self, str: &str) -> Vec<u64> {
        match str {
            "topstories" => self.topstories.clone(),
            "askstories" => self.askstories.clone(),
            "showstories" => self.showstories.clone(),
            "jobstories" => self.jobstories.clone(),
            _ => vec![],
        }
    }
}

fn App(cx: Scope) -> Element {
    // init hackernews list
    let v = use_future(&cx, (), |_| async move {
        let api_list = vec!["topstories", "askstories", "showstories", "jobstories"];
        let mut result_list: Vec<Vec<u64>> = vec![];
        for name in api_list {
            let result = get_list(name).await;
            result_list.push(result);
        }
        result_list
    });
    match v.value() {
        Some(data) => {
            let data = data.clone();
            cx.use_hook(|_| {
                let list = LoadedInfo {
                    topstories: data[0].clone(),
                    askstories: data[1].clone(),
                    showstories: data[2].clone(),
                    jobstories: data[3].clone(),
                };
                log::info!("Loaded Info: {:?}", list);
                cx.provide_context(list);
            });

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
                            ListShow {
                                name: "topstories"
                            }
                        }
                        Route {
                            to: "",
                            pages::_404::NotFound {}
                        }
                    }
                }
            })
        }
        None => None,
    }
}
