use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ConsolePageProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(ConsolePage)]
pub fn console_page(props: &ConsolePageProps) -> Html {
    let mut classes = classes!("console-page");
    if let Some(variant) = &props.variant {
        classes.push(format!("console-page-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
