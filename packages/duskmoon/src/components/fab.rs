use super::popover::popover_anchor_name;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FabProps {
    pub id: AttrValue,
    pub trigger_label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_else(|| classes!("btn", "btn-primary"))]
    pub trigger_class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
}

/// Native speed dial. Use `fab-contained` inside a positioned container.
#[function_component(Fab)]
pub fn fab(props: &FabProps) -> Html {
    fab_view(props)
}

fn fab_view(props: &FabProps) -> Html {
    let anchor = popover_anchor_name(&props.id);
    html! {
        <div class={classes!("fab", "fab-speed-dial", props.class.clone())}>
            <button type="button" class={classes!("fab-trigger", props.trigger_class.clone())}
                popovertarget={props.id.clone()} disabled={props.disabled}
                style={format!("anchor-name: {anchor};")}>
                {props.trigger_label.clone()}
            </button>
            <div id={props.id.clone()} class="fab-actions" popover="auto" style={format!("position-anchor: {anchor};")}>
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
    fn native_actions_share_a_safe_anchor_with_the_trigger() {
        let props = FabProps {
            id: "actions:编辑".into(),
            trigger_label: "Create".into(),
            class: Classes::new(),
            trigger_class: classes!("btn"),
            children: Children::default(),
            disabled: false,
        };
        let VNode::VTag(root) = fab_view(&props) else {
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
        let anchor = popover_anchor_name(&props.id);
        assert!(trigger
            .attributes
            .iter()
            .any(|(key, value)| key == "style" && value == format!("anchor-name: {anchor};")));
        assert!(surface
            .attributes
            .iter()
            .any(|(key, value)| key == "style" && value == format!("position-anchor: {anchor};")));
        assert!(surface
            .attributes
            .iter()
            .any(|(key, value)| key == "popover" && value == "auto"));
        assert!(!root
            .attributes
            .iter()
            .any(|(key, value)| key == "class" && value.contains("fab-open")));
    }
}
