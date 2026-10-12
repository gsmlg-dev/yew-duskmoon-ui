use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Properties, Clone, PartialEq)]
pub struct InputProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub aria_label: AttrValue,
    #[prop_or_default]
    pub input_type: Option<AttrValue>,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    input_view(props)
}

fn input_view(props: &InputProps) -> Html {
    let mut classes = classes!("input");
    if let Some(variant) = &props.variant {
        classes.push(format!("input-{}", variant));
    }
    classes.push(props.class.clone());

    if props.children.iter().any(|child| !is_text_only(&child)) {
        html! { <div class={classes!("form-group", props.class.clone())}>{ for props.children.iter() }</div> }
    } else {
        html! {
            <input class={classes} type={props.input_type.clone().unwrap_or_else(|| "text".into())}
                id={props.id.clone()} name={props.name.clone()} placeholder={props.placeholder.clone()}
                aria-label={props.aria_label.clone()} value={props.value.clone().or_else(|| {
                    let text = children_text(&props.children);
                    (!text.is_empty()).then_some(text)
                })} disabled={props.disabled}
                required={props.required} readonly={props.readonly} oninput={props.oninput.clone()}
                onchange={props.onchange.clone()} />
        }
    }
}

pub(crate) fn is_text_only(node: &VNode) -> bool {
    match node {
        VNode::VText(_) => true,
        VNode::VList(children) => children.iter().all(is_text_only),
        _ => false,
    }
}

pub(crate) fn has_element(node: &VNode, name: &str) -> bool {
    match node {
        VNode::VTag(tag) => {
            tag.tag() == name
                || tag
                    .children()
                    .is_some_and(|children| has_element(children, name))
        }
        VNode::VList(children) => children.iter().any(|child| has_element(child, name)),
        _ => false,
    }
}

pub(crate) fn children_text(children: &Children) -> AttrValue {
    fn push_text(node: &VNode, output: &mut String) {
        match node {
            VNode::VText(text) => output.push_str(&text.text),
            VNode::VList(children) => {
                for child in children.iter() {
                    push_text(child, output);
                }
            }
            VNode::VTag(tag) => {
                if let Some(children) = tag.children() {
                    push_text(children, output);
                }
            }
            _ => {}
        }
    }
    let mut output = String::new();
    for child in children.iter() {
        push_text(&child, &mut output);
    }
    output.into()
}

pub(crate) fn optional_checked(mut node: Html, checked: Option<bool>) -> Html {
    if checked.is_none() {
        if let VNode::VTag(tag) = &mut node {
            std::rc::Rc::make_mut(tag).preserve_checked();
        }
    }
    node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[function_component(CustomInput)]
    fn custom_input() -> Html {
        html! { <input class="input" /> }
    }

    #[test]
    fn preserves_component_children_in_manual_composition() {
        let child = html! { <CustomInput /> };
        let props = InputProps {
            children: Children::new(vec![child.clone()]),
            ..yew::props! { InputProps {} }
        };
        let VNode::VTag(wrapper) = input_view(&props) else {
            panic!("expected composition wrapper");
        };
        assert_eq!(wrapper.tag(), "div");
        assert_eq!(wrapper.children(), Some(&html! { { for [child] } }));
    }

    #[test]
    fn structured_children_are_not_flattened_into_a_native_value() {
        assert!(!is_text_only(&html! { <span>{ "Label" }</span> }));
        assert!(!is_text_only(&html! { <CustomInput /> }));
        assert!(is_text_only(&html! { <>{ "initial" }{ " value" }</> }));
        let VNode::VTag(input) = input_view(&yew::props! { InputProps {} }) else {
            panic!("expected native input");
        };
        assert_eq!(input.tag(), "input");
    }
    #[test]
    fn omitted_checked_state_preserves_native_toggling() {
        let unchecked = optional_checked(html! { <input type="checkbox" checked={false} /> }, None);
        let checked = optional_checked(
            html! { <input type="checkbox" checked={true} /> },
            Some(true),
        );
        let VNode::VTag(unchecked) = unchecked else {
            panic!("expected native input")
        };
        let VNode::VTag(checked) = checked else {
            panic!("expected native input")
        };
        assert_eq!(unchecked.checked(), None);
        assert_eq!(checked.checked(), Some(true));
    }
    #[test]
    fn detects_nested_native_controls_for_legacy_composition() {
        assert!(has_element(
            &html! { <label><input type="text" /></label> },
            "input"
        ));
        assert!(!has_element(&html! { "initial value" }, "input"));
    }
    #[test]
    fn preserves_text_children_as_initial_native_value() {
        assert_eq!(
            children_text(&Children::new(vec![
                html! { "first" },
                html! { <span>{ " second" }</span> }
            ])),
            "first second"
        );
    }
}
