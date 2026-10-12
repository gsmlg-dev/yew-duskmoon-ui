use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct StackProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Stack)]
pub fn stack(props: &StackProps) -> Html {
    let mut classes = classes!("stack");
    if let Some(variant) = &props.variant {
        classes.push(format!("stack-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
