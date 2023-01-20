use regex::bytes::Regex;
use std::fmt::Write;
use sycamore::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Code {
    code: &'static str,
    desc: &'static str,
}

fn main() {
    sycamore::render(|cx| {
        let pattern = create_signal(cx, String::new());
        let subject = create_signal(cx, String::new());

        let result = create_memo(cx, || {
            let pattern = pattern.get().to_string();
            let subject = subject.get().to_string();

            run_regex(&pattern, &subject)
        });

        let reference = create_signal(cx, get_reference());
        let modifiers = create_signal(cx, get_modifiers());

        view! { cx,
            div(class="bg-slate-800 container max-w-none m-0 px-4 pt-10 font-sans") {
                header(class="container text-white text-center space-y-4 px-1") {
                    p(class="text-4xl") { "Rustexp-sycamore" }
                    p(class="text-lg") { "A Rust regular expression editor & tester" }
                }

                div(class="container resize-y flex max-w-5xl items-stretch font-mono leading-snug space-x-4 mt-8") {
                    div(class="container space-y-1 px-1") {
                        div {
                            label(for="pattern", class="block text-white"){ "Regex" }
                            textarea(bind:value=pattern, class="bg-slate-600 text-slate-300 p-1 w-full", name="pattern", rows="1") {}
                        }
                        div {
                            label(for="subject", class="block text-white")  {"Subject"}
                            textarea(bind:value=subject, class="bg-slate-600 text-slate-300 p-1 w-full", name="subject", rows="5") {}
                        }
                    }
                    div(class="container overflow-auto bg-slate-700 text-slate-300 p-2 mt-6 text-white") {
                        pre(class="h-full") {(*result.get())}
                    }
                }

                div(class="container max-w-5xl text-slate-400 text-left font-mono mt-10 pb-8 px-1") {
                    p(class="mt-8 text-white") {"Reference:"}
                    ul(class="list-none mt-4 columns-1 md:columns-3 pl-2") {
                        Indexed(
                            iterable=reference,
                            view=|cx, Code { code, desc }| view! { cx,
                                li { code(class="bg-slate-700 leading-relaxed mr-2 px-1 whitespace-nowrap") {(code)} (desc) }
                            }
                        )
                    }
                    p(class="mt-8 text-white") {
                        "Modifiers (enable: "
                        code(class="bg-slate-700 leading-relaxed mr-2 px-1 whitespace-nowrap"){"(?a)"} ", disable: "
                        code(class="bg-slate-700 leading-relaxed mr-2 px-1 whitespace-nowrap"){"(?-a)"} "}):"
                    }
                    ul(class="list-none mt-4 columns-1 md:columns-3 pl-2") {
                        Indexed(
                            iterable=modifiers,
                            view=|cx, Code { code, desc }| view! { cx,
                                li { code(class="bg-slate-700 leading-relaxed mr-2 px-1 whitespace-nowrap") {(code)} (desc) }
                            }
                        )
                    }
                    p(class="mt-8 text-center") {
                        "For more information see the "
                        a(href="https://docs.rs/regex/", class="text-white", target="_blank") {
                            "documentation for the regex crate"
                        }"."
                    }
                }
            }
            footer(class="container max-w-none m-0 py-8 px-1 bg-slate-700 text-slate-400 text-center") {
                p {
                    "Rustexp-sycamore is " a(href="http://en.wikipedia.org/wiki/Free_software", class="text-white", target="_blank") {
                        "Free software"} ", available under the "
                    a(href="https://gnu.org/licenses/agpl.html", class="text-white", target="_blank") {"GNU
                        AGPL3"} " licence."
                }
                p {
                    "The source code is freely available on "
                    a(href="https://github.com/wr8fdy/rustexp-sycamore", class="text-white", target="_blank") {"GitHub" } "."
                }
                p {
                    "Inspired by Louis Pilfold's excellent "
                    a(href="https://rustexp.lpil.uk/", class="text-white", target="_blank") { "Rustexp" }"."
                }
                p {
                    "Copyright © 2023 - Present "
                    a(href="https://github.com/wr8fdy", class="text-white", target="_blank") {"wr8fdy"}". All Rights
                    Reserved."
                }
            }
        }
    });
}

fn get_modifiers() -> Vec<Code> {
    vec![
        Code {
            code: "u",
            desc: "unicode",
        },
        Code {
            code: "i",
            desc: "case insensitive",
        },
        Code {
            code: "s",
            desc: "dot matches newline",
        },
        Code {
            code: "m",
            desc: "multiline",
        },
        Code {
            code: "x",
            desc: "whitespace ignored",
        },
        Code {
            code: "f",
            desc: "start on first line",
        },
        Code {
            code: "r",
            desc: "inverts greediness",
        },
    ]
}

fn get_reference() -> Vec<Code> {
    vec![
        Code {
            code: ".",
            desc: "non-newline char",
        },
        Code {
            code: "^",
            desc: "start of line",
        },
        Code {
            code: "$",
            desc: "end of line",
        },
        Code {
            code: "\\b",
            desc: "word boundary",
        },
        Code {
            code: "\\B",
            desc: "non-word boundary",
        },
        Code {
            code: "\\A",
            desc: "start of subject",
        },
        Code {
            code: "\\z",
            desc: "end of subject",
        },
        Code {
            code: "\\d",
            desc: "decimal digit",
        },
        Code {
            code: "\\D",
            desc: "non-decimal digit",
        },
        Code {
            code: "\\s",
            desc: "whitespace",
        },
        Code {
            code: "\\S",
            desc: "non-whitespace",
        },
        Code {
            code: "\\w",
            desc: "word character",
        },
        Code {
            code: "\\W",
            desc: "non-word character",
        },
        Code {
            code: "(a|z)",
            desc: "a or z",
        },
        Code {
            code: "[az]",
            desc: "a or z",
        },
        Code {
            code: "[^az]",
            desc: "not a or z",
        },
        Code {
            code: "[a-z]",
            desc: "a through z",
        },
        Code {
            code: "(foo)",
            desc: "capture foo",
        },
        Code {
            code: "a?",
            desc: "0 or 1 a",
        },
        Code {
            code: "a*",
            desc: "0 or more a",
        },
        Code {
            code: "a+",
            desc: "1 or more a",
        },
        Code {
            code: "a{3}",
            desc: "3 of a",
        },
        Code {
            code: "a{3,}",
            desc: "3 or more a",
        },
        Code {
            code: "a{3,5}",
            desc: "3 through 5 a",
        },
    ]
}

fn run_regex(pattern: &str, subject: &str) -> String {
    if pattern.is_empty() {
        return String::new();
    }

    let regex = match Regex::new(&pattern) {
        Err(e) => {
            return e.to_string();
        }
        Ok(re) => re,
    };

    let formatted = format_captures(regex, &subject);
    formatted
}

fn format_captures(regex: regex::bytes::Regex, subject: &str) -> String {
    let mut buffer = String::new();

    for captures in regex.captures_iter(subject.as_bytes()) {
        write!(&mut buffer, "Some(Captures({{\n").unwrap();

        for (i, cap) in captures.iter().enumerate() {
            write!(
                &mut buffer,
                "    {}: Some({:#?}),\n",
                i,
                std::str::from_utf8(cap.unwrap().as_bytes()).unwrap()
            )
            .unwrap();
        }
        write!(&mut buffer, "}})),\n").unwrap();
    }

    if buffer == "" {
        String::from("None")
    } else {
        buffer
    }
}
