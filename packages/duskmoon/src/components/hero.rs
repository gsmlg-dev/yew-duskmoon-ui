use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct HeroProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Hero)]
pub fn hero(props: &HeroProps) -> Html {
    let mut classes = classes!("hero");
    if let Some(variant) = &props.variant {
        classes.push(format!("hero-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <section class={classes}>
            { for props.children.iter() }
        </section>
    }
}
