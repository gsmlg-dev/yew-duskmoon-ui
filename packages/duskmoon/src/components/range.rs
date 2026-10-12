use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct RangeProps {
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
    #[prop_or(AttrValue::from("0"))]
    pub min: AttrValue,
    #[prop_or(AttrValue::from("100"))]
    pub max: AttrValue,
    #[prop_or(AttrValue::from("1"))]
    pub step: AttrValue,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(Range)]
pub fn range(props: &RangeProps) -> Html {
    let mut classes = classes!("range");
    if let Some(variant) = &props.variant {
        classes.push(format!("range-{}", variant));
    }
    classes.push(props.class.clone());
    html! {
        <input class={classes} type="range" id={props.id.clone()} name={props.name.clone()}
            aria-label={props.aria_label.clone()} min={props.min.clone()} max={props.max.clone()}
            step={props.step.clone()} value={props.value.clone()} disabled={props.disabled}
            required={props.required} oninput={props.oninput.clone()} onchange={props.onchange.clone()} />
    }
}
