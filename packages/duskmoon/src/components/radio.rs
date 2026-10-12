use super::input::optional_checked;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, Clone, PartialEq)]
pub struct RadioProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or(AttrValue::from("on"))]
    pub value: AttrValue,
    #[prop_or_default]
    pub checked: Option<bool>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
    #[prop_or(AttrValue::from("Radio options"))]
    pub aria_label: AttrValue,
}

#[function_component(Radio)]
pub fn radio(props: &RadioProps) -> Html {
    let mut classes = classes!("radio");
    if let Some(variant) = &props.variant {
        classes.push(format!("radio-{}", variant));
    }
    classes.push(props.class.clone());
    if props.children.is_empty() {
        optional_checked(
            html! {
                <input class={classes} type="radio" id={props.id.clone()} name={props.name.clone()}
                    aria-label={props.aria_label.clone()} value={props.value.clone()} checked={props.checked.unwrap_or_default()}
                    disabled={props.disabled} required={props.required} onchange={props.onchange.clone()} />
            },
            props.checked,
        )
    } else {
        html! {
            <div class={classes!("radio-group", props.class.clone())} role="radiogroup" aria-label={props.aria_label.clone()}>
                { for props.children.iter() }
            </div>
        }
    }
}
