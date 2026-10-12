use super::popover::popover_anchor_name;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MegamenuProps {
    pub id: AttrValue,
    pub trigger_label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub panel_class: Classes,
    #[prop_or_default]
    pub children: Children,
}

/// Navigation disclosure composed with Core Menu/Link child markup.
#[function_component(Megamenu)]
pub fn megamenu(props: &MegamenuProps) -> Html {
    megamenu_view(props)
}

fn megamenu_view(props: &MegamenuProps) -> Html {
    let style = format!("--megamenu-anchor: {};", popover_anchor_name(&props.id));
    html! {
        <nav class={classes!("megamenu", props.class.clone())} aria-label={props.trigger_label.clone()}>
            <ul class="megamenu-bar">
                <li>
                    <button type="button" class="megamenu-trigger" popovertarget={props.id.clone()} style={style.clone()}>
                        {props.trigger_label.clone()}
                    </button>
                    <div id={props.id.clone()} popover="auto" class={classes!("megamenu-panel", props.panel_class.clone())} {style}>
                        {for props.children.iter()}
                    </div>
                </li>
            </ul>
        </nav>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::virtual_dom::VNode;

    #[test]
    fn each_panel_uses_the_unique_trigger_anchor() {
        let props = MegamenuProps {
            id: "products".into(),
            trigger_label: "Products".into(),
            class: Classes::new(),
            panel_class: Classes::new(),
            children: Children::default(),
        };
        let VNode::VTag(root) = megamenu_view(&props) else {
            panic!("expected nav");
        };
        let Some(VNode::VTag(bar)) = root.children() else {
            panic!("expected bar");
        };
        let Some(VNode::VTag(item)) = bar.children() else {
            panic!("expected list item");
        };
        let Some(VNode::VList(children)) = item.children() else {
            panic!("expected disclosure children");
        };
        for child in children.iter() {
            let VNode::VTag(tag) = child else {
                panic!("expected tag");
            };
            assert!(tag.attributes.iter().any(|(key, value)| key == "style"
                && value == "--megamenu-anchor: --popover-s-products;"));
        }
        let VNode::VTag(panel) = &children[1] else {
            panic!("expected panel");
        };
        assert!(panel
            .attributes
            .iter()
            .any(|(key, value)| key == "popover" && value == "auto"));
    }
}
