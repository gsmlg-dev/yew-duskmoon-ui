use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct BottomNavigationProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(BottomNavigation)]
pub fn bottom_navigation(props: &BottomNavigationProps) -> Html {
    bottom_navigation_view(props)
}

fn bottom_navigation_view(props: &BottomNavigationProps) -> Html {
    let mut classes = classes!("bottom-nav", "bottom-navigation");
    if let Some(variant) = &props.variant {
        classes.push(format!("bottom-nav-{variant}"));
        classes.push(format!("bottom-navigation-{variant}"));
    }
    classes.push(props.class.clone());

    html! {
        <nav class={classes}>
            { for props.children.iter() }
        </nav>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::virtual_dom::VNode;

    #[test]
    fn emits_core_navigation_classes_and_retains_legacy_classes() {
        let props = BottomNavigationProps {
            class: Classes::new(),
            children: Children::default(),
            variant: Some("primary".into()),
        };
        let VNode::VTag(tag) = bottom_navigation_view(&props) else {
            panic!("expected navigation");
        };
        assert_eq!(tag.tag(), "nav");
        let class = tag
            .attributes
            .iter()
            .find_map(|(key, value)| (key == "class").then_some(value))
            .unwrap();
        for required in [
            "bottom-nav",
            "bottom-nav-primary",
            "bottom-navigation",
            "bottom-navigation-primary",
        ] {
            assert!(class.split_whitespace().any(|class| class == required));
        }
    }
}
