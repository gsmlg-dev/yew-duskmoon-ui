use std::rc::Rc;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Properties, Clone, PartialEq)]
pub struct ChatProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
    let mut classes = classes!("chat");
    if let Some(variant @ ("start" | "end")) = props.variant.as_deref() {
        classes.push(format!("chat-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter().map(|mut child| {
                if let Some(variant) = props.variant.as_deref().filter(|variant| !matches!(*variant, "start" | "end")) {
                    apply_bubble_variant(&mut child, variant);
                }
                child
            }) }
        </div>
    }
}

fn apply_bubble_variant(node: &mut VNode, variant: &str) {
    match node {
        VNode::VTag(tag) => {
            let tag = Rc::make_mut(tag);
            let class = tag
                .attributes
                .iter()
                .find_map(|(name, value)| (name == "class").then(|| value.to_owned()));
            if let Some(class) =
                class.filter(|class| class.split_whitespace().any(|token| token == "chat-bubble"))
            {
                tag.add_attribute("class", format!("{} chat-bubble-{}", class, variant));
            }
            if let Some(children) = tag.children_mut() {
                apply_bubble_variant(children, variant);
            }
        }
        VNode::VList(children) => {
            for child in Rc::make_mut(children).iter_mut() {
                apply_bubble_variant(child, variant);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_color_is_applied_to_the_bubble_slot() {
        let mut bubble = html! { <div class="chat-bubble custom">{ "Hello" }</div> };
        apply_bubble_variant(&mut bubble, "secondary");
        let VNode::VTag(tag) = bubble else {
            panic!("expected bubble")
        };
        assert!(tag
            .attributes
            .iter()
            .any(|(name, value)| name == "class"
                && value == "chat-bubble custom chat-bubble-secondary"));
    }
}
