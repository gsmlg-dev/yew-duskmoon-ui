use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct NestedMenuProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(NestedMenu)]
pub fn nested_menu(props: &NestedMenuProps) -> Html {
    nested_menu_view(props)
}

fn nested_menu_view(props: &NestedMenuProps) -> Html {
    let mut classes = classes!("nested-menu");
    if let Some(variant) = &props.variant {
        classes.push(format!("nested-menu-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <ul class={classes}>
            { for props.children.iter() }
        </ul>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::virtual_dom::VNode;

    #[test]
    fn root_is_a_list_for_native_nested_navigation_items() {
        let props = NestedMenuProps {
            class: Classes::new(),
            children: Children::default(),
            variant: None,
        };
        let VNode::VTag(tag) = nested_menu_view(&props) else {
            panic!("expected list");
        };
        assert_eq!(tag.tag(), "ul");
    }
}
