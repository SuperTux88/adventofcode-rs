use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use web_sys::HtmlSelectElement;
use yew::{function_component, html, use_node_ref, Callback, Html, Properties, ToHtml};

#[derive(Properties, PartialEq)]
pub struct SelectProps<T>
where
    T: PartialEq,
{
    pub options: Vec<T>,
    pub selected: Option<T>,
    pub onchange: Option<Callback<T>>,
}

#[function_component]
pub fn Select<T>(props: &SelectProps<T>) -> Html
where
    T: PartialEq + Display + FromStr + ToHtml + 'static,
    <T as FromStr>::Err: Debug,
{
    let selected = props.selected.as_ref();
    let options = props
        .options
        .iter()
        .map(|option| {
            html! {
                <option selected={selected == Some(option)}>{option}</option>
            }
        })
        .collect::<Html>();

    let select_node_ref = use_node_ref();
    let onchange = props.onchange.as_ref().map(|onchange| {
        let select_node_ref = select_node_ref.clone();
        let onchange = onchange.clone();

        move |_| {
            if let Some(select) = select_node_ref.cast::<HtmlSelectElement>() {
                onchange.emit(select.value().parse().unwrap());
            }
        }
    });

    html! {
        <select ref={select_node_ref} {onchange}>{options}</select>
    }
}
