use yew::prelude::*;

/// Native validation composition. Children opt in with `.validator`; errors use `.validator-error`.
/// `immediate` enables `.validate` styling before user interaction.
#[derive(Properties, Clone, PartialEq)]
pub struct ValidatorProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub immediate: bool,
}

#[function_component(Validator)]
pub fn validator(props: &ValidatorProps) -> Html {
    let mut classes = classes!("form-group");
    if let Some(variant) = &props.variant {
        classes.push(format!("form-group-{}", variant));
    }
    classes.push(props.class.clone());
    if props.immediate {
        classes.push("validate");
    }
    html! {
        <div class={classes}>{ for props.children.iter() }</div>
    }
}
