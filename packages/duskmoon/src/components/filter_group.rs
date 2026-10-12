use yew::prelude::*;

/// Compose direct `label.chip` children containing native `.filter-group-input` controls.
#[derive(Properties, Clone, PartialEq)]
pub struct FilterGroupProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or(AttrValue::from("Filters"))]
    pub aria_label: AttrValue,
}

#[function_component(FilterGroup)]
pub fn filter_group(props: &FilterGroupProps) -> Html {
    let mut classes = classes!("filter-group");
    if let Some(variant) = &props.variant {
        classes.push(format!("filter-group-{}", variant));
    }
    classes.push(props.class.clone());
    html! {
        <div class={classes} role="group" aria-label={props.aria_label.clone()}>
            { for props.children.iter() }
        </div>
    }
}
