pub use dioxus::prelude::*;

pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        nav {
            class: "bg-gray-800",
            div {
                class: "max-w-7xl mx-auto px-2 sm:px-6 lg:px-8",
                div {
                    class: "relative flex items-center justify-between h-16",
                    div {
                        class: "absolute inset-y-0 left-0 flex items-center sm:hidden",
                        button {
                            class: "inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white",
                            "aria-controls": "mobile-menu",
                            "aria-expanded": "false",
                            r#type: "button",span {
                                class: "sr-only",
                                "Open main menu"
                            }
                        }
                    }
                    div {
                        class: "flex-1 flex items-center justify-center sm:items-stretch sm:justify-start",
                        div {
                            class: "flex-shrink-0 flex items-center",
                            img {
                                class: "block lg:hidden h-8 w-auto",
                                alt: "DioxusLabs",
                                src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                            }
                            img {
                                class: "hidden lg:block h-8 w-auto",
                                alt: "DioxusLabs",
                                src: "https://avatars.githubusercontent.com/u/79236386?s=200&v=4",
                            }
                        }
                        div {
                            class: "hidden sm:block sm:ml-6",
                            div {
                                class: "flex space-x-4",
                                Link {
                                    class: "bg-gray-900 text-white px-3 py-2 rounded-md text-sm font-medium",
                                    to: "/",
                                    "News"
                                }
                            }
                        }
                    }
                    div {
                        class: "absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0",
                        div {
                            class: "ml-3 relative",
                            div {
                                class: "text-white font-semibold sm:block",
                                "Built with Dioxus"
                            }
                        }
                    }
                }
            }
            div {
                class: "sm:hidden",
                id: "mobile-menu",
                div {
                    class: "px-2 pt-2 pb-3 space-y-1",
                    Link {
                        class: "bg-gray-900 text-white block px-3 py-2 rounded-md text-base font-medium",
                        to: "/",
                        "News"
                    }
                }
            }
        }
        br {}
    })
}

#[inline_props]
pub fn Pagination(cx: Scope, max_page_num: usize) -> Element {
    let route = use_route(&cx);

    let current_page_num = route.segment("page").unwrap();
    let mut current_page_num = current_page_num.parse::<usize>().unwrap_or(1);

    if current_page_num <= 0 {
        current_page_num = 1;
    }
    if current_page_num > *max_page_num {
        current_page_num = *max_page_num;
    }
    
    let next_page_num = if current_page_num < *max_page_num {
        current_page_num + 1
    } else {
        current_page_num
    };
    let prev_page_num = if current_page_num > 1 {
        current_page_num - 1
    } else {
        current_page_num
    };

    cx.render(rsx! {
        div {
            class: "flex justify-center",
            nav {
                "aria-label": "Page navigation example",
                ul {
                    class: "flex list-style-none",
                    li {
                        class: "page-item",
                        Link {
                            class: "page-link relative block py-1.5 px-3 rounded border-0 bg-transparent outline-none transition-all duration-300 rounded text-gray-800 hover:text-gray-800 hover:bg-gray-200 focus:shadow-none",
                            to: "{prev_page_num}",
                            span {
                                "<<"
                            }
                        }
                    }
                    li {
                        class: "page-item",
                        span {
                            class: "page-link relative block py-1.5 px-3 rounded border-0 bg-transparent outline-none transition-all duration-300 rounded text-gray-800",
                            span {
                                "{current_page_num} / {max_page_num}"
                            }
                        }
                    }
                    li {
                        class: "page-item",
                        Link {
                            class: "page-link relative block py-1.5 px-3 rounded border-0 bg-transparent outline-none transition-all duration-300 rounded text-gray-800 hover:text-gray-800 hover:bg-gray-200 focus:shadow-none",
                            to: "{next_page_num}",
                            span {
                                ">>"
                            }
                        }
                    }
                }
            }
        }
    })
}
