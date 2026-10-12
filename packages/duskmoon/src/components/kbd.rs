use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct KbdProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Kbd)]
pub fn kbd(props: &KbdProps) -> Html {
    let mut classes = classes!("kbd");
    if let Some(variant) = &props.variant {
        classes.push(format!("kbd-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <kbd class={classes}>
            { for props.children.iter() }
        </kbd>
    }
}
