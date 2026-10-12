use super::input::{children_text, has_element, is_text_only};
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SelectProps {
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
    pub aria_label: AttrValue,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    /// Force manual composition when an opaque child component renders a full select.
    #[prop_or_default]
    pub composition: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let node_ref = use_node_ref();
    let select_ref = node_ref.clone();
    let value = props.value.clone();
    // A select has no native value attribute. Synchronize its property after
    // its options mount, and after every controlled render. None leaves the
    // browser's selection intact; a single value selects one option, even
    // when multiple selection is enabled.
    use_effect(move || {
        if let (Some(select), Some(value)) = (select_ref.cast::<HtmlSelectElement>(), value) {
            select.set_value(&value);
        }
        || ()
    });
    select_view(props, node_ref)
}

fn select_view(props: &SelectProps, node_ref: NodeRef) -> Html {
    let mut classes = classes!("select");
    if let Some(variant) = &props.variant {
        classes.push(format!("select-{}", variant));
    }
    classes.push(props.class.clone());

    if props.composition
        || props
            .children
            .iter()
            .any(|child| has_element(&child, "select"))
    {
        html! { <div class={classes!("select-container", props.class.clone())}>{ for props.children.iter() }</div> }
    } else {
        let text_only = props.children.iter().all(|child| is_text_only(&child));
        html! {
            <select ref={node_ref} class={classes} id={props.id.clone()} name={props.name.clone()} aria-label={props.aria_label.clone()}
                disabled={props.disabled} required={props.required}
                multiple={props.multiple} onchange={props.onchange.clone()}>
                { if text_only { html! { <option>{ children_text(&props.children) }</option> } }
                  else { html! { <>{ for props.children.iter() }</> } } }
            </select>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::virtual_dom::VNode;

    #[function_component(CustomOptions)]
    fn custom_options() -> Html {
        html! { <option value="b">{ "B" }</option> }
    }

    #[test]
    fn preserves_component_options_inside_the_native_select() {
        let child = html! { <CustomOptions /> };
        let props = SelectProps {
            children: Children::new(vec![child.clone()]),
            ..yew::props! { SelectProps {} }
        };
        let VNode::VTag(select) = select_view(&props, NodeRef::default()) else {
            panic!("expected native select");
        };
        assert_eq!(select.tag(), "select");
        assert_eq!(select.children(), Some(&html! { <>{ for [child] }</> }));
    }

    #[test]
    fn explicit_composition_preserves_opaque_full_control_children() {
        let child = html! { <CustomOptions /> };
        let props = SelectProps {
            composition: true,
            children: Children::new(vec![child.clone()]),
            ..yew::props! { SelectProps {} }
        };
        let VNode::VTag(wrapper) = select_view(&props, NodeRef::default()) else {
            panic!("expected composition wrapper");
        };
        assert_eq!(wrapper.tag(), "div");
        assert_eq!(wrapper.children(), Some(&html! { { for [child] } }));
    }

    #[test]
    fn known_native_select_children_still_use_manual_composition() {
        let props = SelectProps {
            children: Children::new(vec![html! { <label><select /></label> }]),
            ..yew::props! { SelectProps {} }
        };
        let VNode::VTag(wrapper) = select_view(&props, NodeRef::default()) else {
            panic!("expected composition wrapper");
        };
        assert_eq!(wrapper.tag(), "div");
    }
}
