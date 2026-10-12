use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FormProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub onsubmit: Callback<SubmitEvent>,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let mut classes = classes!("form-group");
    if let Some(variant) = &props.variant {
        classes.push(format!("form-group-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <form class={classes} onsubmit={props.onsubmit.clone()}>
            { for props.children.iter() }
        </form>
    }
}
