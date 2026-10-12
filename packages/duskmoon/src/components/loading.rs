use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct LoadingProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or(AttrValue::from("spinner"))]
    pub indicator: AttrValue,
    #[prop_or_default]
    pub size: Option<String>,
    #[prop_or(AttrValue::from("Loading"))]
    pub aria_label: AttrValue,
}

#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    let mut classes = classes!("loading");
    if let Some(variant) = &props.variant {
        classes.push(format!("loading-{}", variant));
    }
    classes.push(props.class.clone());
    classes.push(format!("loading-{}", props.indicator));
    if let Some(size) = &props.size {
        classes.push(format!("loading-{}", size));
    }
    html! {
        <span class={classes} role="status" aria-label={props.aria_label.clone()} />
    }
}
