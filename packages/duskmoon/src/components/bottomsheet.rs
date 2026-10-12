use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct BottomsheetProps {
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

#[function_component(Bottomsheet)]
pub fn bottomsheet(props: &BottomsheetProps) -> Html {
    bottomsheet_view(props)
}

fn bottomsheet_view(props: &BottomsheetProps) -> Html {
    let mut classes = classes!("bottomsheet");
    if let Some(variant) = &props.variant {
        classes.push(format!("bottomsheet-{variant}"));
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
        let mut props = BottomsheetProps {
            id: Some("sheet".into()),
            modal: true,
            aria_label: Some("Actions".into()),
            class: Classes::new(),
            children: Children::default(),
            variant: None,
        };
        let VNode::VTag(dialog) = bottomsheet_view(&props) else {
            panic!("expected dialog");
        };
        assert_eq!(dialog.tag(), "dialog");
        assert!(dialog
            .attributes
            .iter()
            .any(|(key, value)| key == "id" && value == "sheet"));
        assert!(!dialog.attributes.iter().any(|(key, _)| key == "open"));
        props.modal = false;
        let VNode::VTag(popover) = bottomsheet_view(&props) else {
            panic!("expected popover");
        };
        assert_eq!(popover.tag(), "div");
        assert!(popover
            .attributes
            .iter()
            .any(|(key, value)| key == "popover" && value == "auto"));
    }
}
