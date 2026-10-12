use super::input::optional_checked;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SwitchProps {
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

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let mut classes = classes!("switch");
    if let Some(variant) = &props.variant {
        classes.push(format!("switch-{}", variant));
    }
    classes.push(props.class.clone());
    if props.children.is_empty() {
        optional_checked(
            html! {
                <input class={classes} type="checkbox" id={props.id.clone()} name={props.name.clone()}
                    aria-label={props.aria_label.clone()} value={props.value.clone()} checked={props.checked.unwrap_or_default()}
                    disabled={props.disabled} required={props.required} onchange={props.onchange.clone()} />
            },
            props.checked,
        )
    } else {
        html! {
            <div class={classes!("switch-group", props.class.clone())}>
                { for props.children.iter() }
            </div>
        }
    }
}
