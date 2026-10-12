use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct StatProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Stat)]
pub fn stat(props: &StatProps) -> Html {
    let mut classes = classes!("stat");
    if let Some(variant) = &props.variant {
        classes.push(format!("stat-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
