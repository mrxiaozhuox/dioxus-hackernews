use dioxus::prelude::*;

use crate::{
    api::{get_item, split_page},
    components::nav::Pagination,
    LoadedInfo,
};

#[inline_props]
pub fn ListShow<'a>(cx: Scope, name: &'a str) -> Element {
    let name = name.to_string();
    let page = use_route(&cx).segment("page").unwrap_or("1");
    let page = page.parse::<usize>().unwrap_or(1);

    let current_list = cx.consume_context::<LoadedInfo>().unwrap();
    let current_list = current_list.from_str(&name);
    let current_list = split_page(current_list, page);

    let res = use_future(&cx, (), |_| async move {
        let mut list = vec![];
        for id in current_list {
            let res = get_item(id).await;
            if let Some(r) = res {
                list.push(r);
            }
        }
        list
    });

    match res.value() {
        Some(data) => {

            let display_list = data.iter().map(|v| {
                rsx! {
                    li {
                        class: "px-6 py-2 border-b border-gray-200 w-full",
                        key: "{v.id}",
                        div {
                            class: "flex justify-start gap-2",
                            div {
                                class: "flex items-center font-semibold w-10 text-gray-600",
                                "{v.score}"
                            }
                            div {
                                class: "",
                                a {
                                    class: "font-semibold text-lg",
                                    href: "/",
                                    "{v.title}"
                                }
                                p {
                                    class: "text-gray-500",
                                    "by "
                                    span {
                                        class: "underline",
                                        "{v.by}"
                                    }
                                    " | "
                                    Link {
                                        class: "underline hover:text-gray-800",
                                        to: "/comments/{v.id}",
                                        "{v.descendants}"
                                    }
                                    " comments"
                                }
                            }
                        }
                    }
                }
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
                                class: "bg-white rounded-lg w-5/6 text-gray-900",
                                display_list
                            }
                        }
                    }
                }
            })
        }
        None => cx.render(rsx! {
            div {
                class: "flex justify-center",
                h2 {
                    class: "text-2xl font-bold",
                    "Loading..."
                }
            }
        }),
    }
}
