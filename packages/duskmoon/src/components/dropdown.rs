use yew::prelude::*;

/// Logical preferred placement; Core supplies collision fallbacks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DropdownPlacement {
    BlockStart,
    #[default]
    BlockEnd,
    InlineStart,
    InlineEnd,
}

impl DropdownPlacement {
    fn class(self) -> &'static str {
        match self {
            Self::BlockStart => "dropdown-block-start",
            Self::BlockEnd => "dropdown-block-end",
            Self::InlineStart => "dropdown-inline-start",
            Self::InlineEnd => "dropdown-inline-end",
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct DropdownProps {
    pub id: AttrValue,
    pub trigger_label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub content_class: Classes,
    #[prop_or_else(|| classes!("btn", "btn-primary"))]
    pub trigger_class: Classes,
    #[prop_or_default]
    pub placement: DropdownPlacement,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
}

/// Native disclosure with a scoped Core CSS anchor and browser light-dismiss.
#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    dropdown_view(props)
}

fn dropdown_view(props: &DropdownProps) -> Html {
    html! {
        <div class={classes!("dropdown", props.placement.class(), props.class.clone())}>
            <button type="button" class={props.trigger_class.clone()} popovertarget={props.id.clone()} disabled={props.disabled}>
                {props.trigger_label.clone()}
            </button>
            <div id={props.id.clone()} class={classes!("dropdown-content", props.content_class.clone())} popover="auto">
                {for props.children.iter()}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::virtual_dom::VNode;

    #[test]
    fn native_trigger_and_surface_are_direct_children_for_scoped_anchoring() {
        let props = DropdownProps {
            id: "actions".into(),
            trigger_label: "Actions".into(),
            class: Classes::new(),
            content_class: Classes::new(),
            trigger_class: classes!("btn"),
            placement: DropdownPlacement::InlineEnd,
            children: Children::default(),
            disabled: false,
        };
        let VNode::VTag(root) = dropdown_view(&props) else {
            panic!("expected root");
        };
        let Some(VNode::VList(children)) = root.children() else {
            panic!("expected children");
        };
        let VNode::VTag(trigger) = &children[0] else {
            panic!("expected trigger");
        };
        let VNode::VTag(surface) = &children[1] else {
            panic!("expected surface");
        };
        assert!(trigger
            .attributes
            .iter()
            .any(|(key, value)| key == "popovertarget" && value == "actions"));
        assert!(surface
            .attributes
            .iter()
            .any(|(key, value)| key == "popover" && value == "auto"));
        assert!(surface
            .attributes
            .iter()
            .any(|(key, value)| key == "id" && value == "actions"));
        assert!(root
            .attributes
            .iter()
            .any(|(key, value)| key == "class" && value.contains("dropdown-inline-end")));
    }
}
