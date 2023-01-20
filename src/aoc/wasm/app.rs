use web_sys::HtmlTextAreaElement;
use yew::{
    functional::{function_component, use_node_ref, use_state},
    html::Html,
    macros::html,
};

use super::select::Select;

use crate::{
    aoc::{part::Part, run},
    Solutions,
};

struct Selection {
    year: u16,
    day: u8,
    part: Part,
}

#[function_component]
pub fn App() -> Html {
    let selection = use_state(|| {
        let latest_year = Solutions::years().into_iter().max().unwrap();
        Selection {
            year: latest_year,
            day: Solutions::days_for_year(latest_year)
                .into_iter()
                .min()
                .unwrap(),
            part: Part::Both,
        }
    });
    let results = use_state(|| None);

    let on_change_year = {
        let selection = selection.clone();
        let results = results.clone();
        move |year| {
            selection.set(Selection {
                year,
                day: Solutions::days_for_year(year).into_iter().min().unwrap(),
                part: selection.part,
            });
            results.set(None);
        }
    };
    let on_change_day = {
        let selection = selection.clone();
        let results = results.clone();
        move |day| {
            selection.set(Selection {
                year: selection.year,
                day,
                part: selection.part,
            });
            results.set(None);
        }
    };
    let on_change_part = {
        let selection = selection.clone();
        let results = results.clone();
        move |part| {
            selection.set(Selection {
                year: selection.year,
                day: selection.day,
                part,
            });
            results.set(None);
        }
    };

    let input_node_ref = use_node_ref();

    let on_run = {
        let selection = selection.clone();
        let input_node_ref = input_node_ref.clone();
        let results = results.clone();

        move |_| {
            let day = Solutions::get(selection.year, selection.day);
            let input = input_node_ref
                .cast::<HtmlTextAreaElement>()
                .unwrap()
                .value();

            results.set(Some(run::run(day, &selection.part, &mut input.as_bytes())));
        }
    };

    html! {
        <>
            <h1>{ "🎄 Advent of Code - Rust 🦀" }</h1>
            <div>
                { "Selected year: " }
                <Select<u16> options={Solutions::years()} selected={Some(selection.year)} onchange={on_change_year} />
            </div>
            <div>
                { "Selected day: " }
                <Select<u8> options={Solutions::days_for_year(selection.year)} selected={selection.day} onchange={on_change_day} />
            </div>
            <div>
                { "Selected part: " }
                <Select<Part> options={Part::values()} selected={Some(selection.part)} onchange={on_change_part} />
            </div>
            <div>
                { "Input: " }<br />
                <textarea ref={input_node_ref} rows="10" cols="50" />
            </div>
            <div>
                <button onclick={on_run}>{ "Run" }</button>
            </div>
            if let Some(results) = results.as_ref() {
                <div>
                    { "Results for " }{ selection.year }{ " day " }{ selection.day }{ ": " }
                    if let Some(part1) = results.part1.as_ref() {
                        <div>
                            { "Part 1: " }{ part1 }
                        </div>
                    }
                    if let Some(part2) = results.part2.as_ref() {
                        <div>
                            { "Part 2: " }{ part2 }
                        </div>
                    }
                </div>
            }
        </>
    }
}
