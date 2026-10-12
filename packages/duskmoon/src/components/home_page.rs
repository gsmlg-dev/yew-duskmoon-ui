use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct HomePageProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(HomePage)]
pub fn home_page(props: &HomePageProps) -> Html {
    let mut classes = classes!("home-page");
    if let Some(variant) = &props.variant {
        classes.push(format!("home-page-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
