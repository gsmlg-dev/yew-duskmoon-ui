use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DrawerProps {
    /// Stable DOM target for `commandfor` or `popovertarget` invokers.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Modal dialogs use `show-modal`; non-modal sheets use native popovers.
    #[prop_or(true)]
    pub modal: bool,
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    drawer_view(props)
}

fn drawer_view(props: &DrawerProps) -> Html {
    let mut classes = classes!("drawer");
    if let Some(variant) = &props.variant {
        classes.push(format!("drawer-{variant}"));
    }
    classes.push(props.class.clone());

    if props.modal {
        html! {
            <dialog id={props.id.clone()} class={classes} aria-label={props.aria_label.clone()}>
                { for props.children.iter() }
            </dialog>
        }
    } else {
        html! {
            <div id={props.id.clone()} popover="auto" class={classes} aria-label={props.aria_label.clone()}>
                { for props.children.iter() }
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::virtual_dom::VNode;

    #[test]
    fn renders_closed_native_modal_and_non_modal_surfaces() {
        let mut props = DrawerProps {
            id: Some("sheet".into()),
            modal: true,
            aria_label: Some("Actions".into()),
            class: Classes::new(),
            children: Children::default(),
            variant: None,
        };
        let VNode::VTag(dialog) = drawer_view(&props) else {
            panic!("expected dialog");
        };
        assert_eq!(dialog.tag(), "dialog");
        assert!(dialog
            .attributes
            .iter()
            .any(|(key, value)| key == "id" && value == "sheet"));
        assert!(!dialog.attributes.iter().any(|(key, _)| key == "open"));
        props.modal = false;
        let VNode::VTag(popover) = drawer_view(&props) else {
            panic!("expected popover");
        };
        assert_eq!(popover.tag(), "div");
        assert!(popover
            .attributes
            .iter()
            .any(|(key, value)| key == "popover" && value == "auto"));
    }
}
