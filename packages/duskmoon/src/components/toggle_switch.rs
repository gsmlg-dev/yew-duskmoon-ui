use super::input::optional_checked;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ToggleSwitchProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub aria_label: AttrValue,
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
}

#[function_component(ToggleSwitch)]
pub fn toggle_switch(props: &ToggleSwitchProps) -> Html {
    let mut classes = classes!("toggle");
    if let Some(variant) = &props.variant {
        classes.push(format!("toggle-{}", variant));
    }
    classes.push(props.class.clone());
    optional_checked(
        html! {
            <input class={classes} type="checkbox" role="switch" id={props.id.clone()} name={props.name.clone()}
                aria-label={props.aria_label.clone()} value={props.value.clone()} checked={props.checked.unwrap_or_default()}
                disabled={props.disabled} required={props.required} onchange={props.onchange.clone()} />
        },
        props.checked,
    )
}
