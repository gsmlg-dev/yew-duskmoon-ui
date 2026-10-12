use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SignPageProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(SignPage)]
pub fn sign_page(props: &SignPageProps) -> Html {
    let mut classes = classes!("sign-page");
    if let Some(variant) = &props.variant {
        classes.push(format!("sign-page-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
