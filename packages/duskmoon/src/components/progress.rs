use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ProgressProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub value: Option<f64>,
    #[prop_or(100.0)]
    pub max: f64,
    #[prop_or(AttrValue::from("Progress"))]
    pub aria_label: AttrValue,
}

#[function_component(Progress)]
pub fn progress(props: &ProgressProps) -> Html {
    let mut classes = classes!("progress");
    if let Some(variant) = &props.variant {
        classes.push(format!("progress-{}", variant));
    }
    classes.push(props.class.clone());

    if props.children.is_empty() {
        html! {
            <progress class={classes} value={props.value.map(|value| value.to_string())}
                max={props.max.to_string()} aria-label={props.aria_label.clone()} />
        }
    } else {
        html! {
            <div class={classes} role="progressbar" aria-label={props.aria_label.clone()}
                aria-valuemin="0" aria-valuemax={props.max.to_string()}
                aria-valuenow={props.value.map(|value| value.to_string())}>
                { for props.children.iter() }
            </div>
        }
    }
}
