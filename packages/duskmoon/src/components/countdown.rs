use yew::prelude::*;

/// Presentation for consumer-owned timer state. Values use ordinary `.countdown-value` text.
#[derive(Properties, Clone, PartialEq)]
pub struct CountdownProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
}

#[function_component(Countdown)]
pub fn countdown(props: &CountdownProps) -> Html {
    let mut classes = classes!("countdown");
    if let Some(variant) = &props.variant {
        classes.push(format!("countdown-{}", variant));
    }
    classes.push(props.class.clone());
    html! {
        <span class={classes} role="timer" aria-label={props.aria_label.clone()}>
            { for props.children.iter() }
        </span>
    }
}
