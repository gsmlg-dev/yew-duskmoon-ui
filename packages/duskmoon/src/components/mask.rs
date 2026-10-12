use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MaskProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Mask)]
pub fn mask(props: &MaskProps) -> Html {
    let mut classes = classes!("mask");
    if let Some(variant) = &props.variant {
        classes.push(format!("mask-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
