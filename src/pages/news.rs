use dioxus::prelude::*;

use crate::{api::get_list, components::nav::Pagination};

#[inline_props]
pub fn ListShow<'a>(cx: Scope, name: &'a str) -> Element {
    let name = name.to_string();
    let task = use_future(&cx, (), |_| async move { 
        let v = get_list(&name).await;
    });

    cx.render(rsx! {
        div {
            class: "flex justify-center",
            div {
                class: "block p-6 rounded-lg shadow-lg bg-white w-full",
                p {
                    class: "text-gray-900 leading-tight font-medium mb-2",
                    Pagination {
                        max_page_num: 10,
                    }
                }
                div {
                    class: "flex justify-center",
                    ul {
                        class: "bg-white rounded-lg w-4/5 text-gray-900",
                        li {
                            class: "px-6 py-2 border-b border-gray-200 w-full",
                            a {
                                class: "font-semibold text-lg",
                                href: "/",
                                "[1000] Nothing Phone (1) – Nothing US"
                            }
                            p {
                                class: "text-gray-500",
                                "by obilgic | 0 comments"
                            }
                        }
                    }
                }
            }
        }
    })
}
