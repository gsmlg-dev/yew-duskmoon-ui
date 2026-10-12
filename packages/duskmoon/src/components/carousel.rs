use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CarouselProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or(AttrValue::from("Carousel"))]
    pub aria_label: AttrValue,
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    let mut classes = classes!("carousel");
    if let Some(variant) = &props.variant {
        classes.push(format!("carousel-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes} role="region" aria-label={props.aria_label.clone()} tabindex="0">
            { for props.children.iter() }
        </div>
    }
}
