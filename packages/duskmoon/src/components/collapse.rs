use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CollapseProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub open: bool,
}

#[function_component(Collapse)]
pub fn collapse(props: &CollapseProps) -> Html {
    collapse_view(props)
}

fn collapse_view(props: &CollapseProps) -> Html {
    let mut classes = classes!("collapse");
    if let Some(variant) = &props.variant {
        classes.push(format!("collapse-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <details class={classes} open={props.open || props.class.contains("collapse-open") || props.class.contains("show")}>
            { for props.children.iter() }
        </details>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::virtual_dom::VNode;

    #[test]
    fn native_disclosure_preserves_legacy_initial_open_classes() {
        let props = CollapseProps {
            class: classes!("collapse-open"),
            children: Children::default(),
            variant: None,
            open: false,
        };
        let VNode::VTag(tag) = collapse_view(&props) else {
            panic!("expected native disclosure");
        };
        assert_eq!(tag.tag(), "details");
        assert!(tag.attributes.iter().any(|(name, _)| name == "open"));
    }
}
