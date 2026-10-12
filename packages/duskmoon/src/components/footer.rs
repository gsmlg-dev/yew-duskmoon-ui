use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let mut classes = classes!("footer");
    if let Some(variant) = &props.variant {
        classes.push(format!("footer-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <footer class={classes}>
            { for props.children.iter() }
        </footer>
    }
}
