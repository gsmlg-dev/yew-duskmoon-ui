use yew::prelude::*;
use yew::virtual_dom::AttrValue;

/// Native Popover API behavior for the generated surface.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverMode {
    #[default]
    Auto,
    Manual,
}

impl PopoverMode {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Manual => "manual",
        }
    }
}

/// Invoker command emitted by the generated trigger button.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PopoverCommand {
    #[default]
    Toggle,
    Show,
    Hide,
}

impl PopoverCommand {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Toggle => "toggle-popover",
            Self::Show => "show-popover",
            Self::Hide => "hide-popover",
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct PopoverProps {
    /// Stable DOM id targeted by the generated HTML invoker command.
    pub id: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub mode: PopoverMode,
    #[prop_or_default]
    pub command: PopoverCommand,
    /// Whether activation invokes `command` on this popover. Set to `false`
    /// when the trigger performs an application action instead.
    #[prop_or(true)]
    pub trigger_command_enabled: bool,
    /// Show descriptive content through native hover/focus interest invokers.
    /// This emits `interestfor` independently of click commands and requires
    /// browser support for the native interest invoker API.
    #[prop_or_default]
    pub trigger_interest: bool,
    #[prop_or(AttrValue::from("Show popover"))]
    pub trigger_label: AttrValue,
    #[prop_or_else(default_trigger_class)]
    pub trigger_class: Classes,
    /// Activation callback on the generated native button. Native keyboard
    /// activation also emits this click event. Commands remain enabled unless
    /// `trigger_command_enabled` is `false` or the callback prevents default.
    #[prop_or_default]
    pub trigger_onclick: Callback<MouseEvent>,
    /// Accessible action name, independent of the visible `trigger_label`.
    /// When omitted, the button's text supplies its accessible name.
    #[prop_or_default]
    pub trigger_aria_label: Option<AttrValue>,
    /// IDs of descriptive content for the trigger, such as this popover's id.
    #[prop_or_default]
    pub trigger_aria_describedby: Option<AttrValue>,
    /// Optional `aria-haspopup` value, such as `"menu"` or `"dialog"`.
    /// Omit it for descriptive content that is not an interactive popup.
    #[prop_or_default]
    pub trigger_aria_haspopup: Option<AttrValue>,
    /// Optional, caller-managed `aria-expanded` state. No value is inferred
    /// from native popover visibility; `None` omits the attribute.
    #[prop_or_default]
    pub trigger_aria_expanded: Option<bool>,
}

/// An anchored native popover with a declarative HTML command trigger.
///
/// Display, light-dismiss, Escape handling, and top-layer behavior are owned
/// by the browser. Use [`PopoverMode::Manual`] when the surface must remain
/// open until an explicit `hide-popover` command is invoked.
///
/// Use `trigger_onclick` with `trigger_command_enabled={false}` for a button
/// whose activation performs an application action. Give it an independent
/// `trigger_aria_label` and, when appropriate, `trigger_aria_describedby`.
/// Popup and expansion ARIA attributes are optional and caller-managed.
/// Set `trigger_interest={true}` to show content on hover/focus in browsers
/// that support native `interestfor`; it does not change the click action.
#[function_component(Popover)]
pub fn popover(props: &PopoverProps) -> Html {
    popover_view(props)
}

fn popover_view(props: &PopoverProps) -> Html {
    let anchor = popover_anchor_name(&props.id);
    let trigger_style: AttrValue = format!("anchor-name: {anchor};").into();
    let surface_style: AttrValue = format!("position-anchor: {anchor};").into();

    html! {
        <>
            <button
                type="button"
                class={props.trigger_class.clone()}
                command={props.trigger_command_enabled.then_some(props.command.as_str())}
                commandfor={props.trigger_command_enabled.then(|| props.id.clone())}
                interestfor={props.trigger_interest.then(|| props.id.clone())}
                onclick={props.trigger_onclick.clone()}
                aria-label={props.trigger_aria_label.clone()}
                aria-describedby={props.trigger_aria_describedby.clone()}
                aria-haspopup={props.trigger_aria_haspopup.clone()}
                aria-expanded={props.trigger_aria_expanded.map(|expanded| expanded.to_string())}
                style={trigger_style}
            >
                { props.trigger_label.clone() }
            </button>
            <div
                id={props.id.clone()}
                popover={props.mode.as_str()}
                class={popover_classes(props)}
                style={surface_style}
            >
                {
                    if props.children.is_empty() {
                        html! { <div class="popover-body">{ "Popover content" }</div> }
                    } else {
                        html! { <>{ for props.children.iter() }</> }
                    }
                }
                <span class="popover-arrow" aria-hidden="true"></span>
            </div>
        </>
    }
}

fn default_trigger_class() -> Classes {
    classes!("btn", "btn-primary")
}

fn popover_classes(props: &PopoverProps) -> Classes {
    let mut classes = classes!("popover");
    if let Some(variant) = &props.variant {
        classes.push(format!("popover-{variant}"));
    }
    classes.push(props.class.clone());
    classes
}

pub(crate) fn popover_anchor_name(id: &str) -> String {
    if id.is_empty() {
        "--popover-empty".to_owned()
    } else if id
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
    {
        format!("--popover-s-{id}")
    } else {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut encoded = String::with_capacity(id.len() * 2);
        for byte in id.bytes() {
            encoded.push(HEX[(byte >> 4) as usize] as char);
            encoded.push(HEX[(byte & 0x0f) as usize] as char);
        }

        format!("--popover-x-{encoded}")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use yew::virtual_dom::{VNode, VTag};

    fn props() -> PopoverProps {
        PopoverProps {
            id: "deployment-options".into(),
            class: classes!("popover-bottom"),
            children: Children::default(),
            variant: Some("primary".to_owned()),
            mode: PopoverMode::Auto,
            command: PopoverCommand::Toggle,
            trigger_command_enabled: true,
            trigger_interest: false,
            trigger_label: "Show options".into(),
            trigger_class: default_trigger_class(),
            trigger_onclick: Callback::default(),
            trigger_aria_label: None,
            trigger_aria_describedby: None,
            trigger_aria_haspopup: None,
            trigger_aria_expanded: None,
        }
    }

    fn tag(node: &VNode) -> &VTag {
        match node {
            VNode::VTag(tag) => tag,
            other => panic!("expected VTag, got {other:?}"),
        }
    }

    fn attrs(tag: &VTag) -> HashMap<&str, &str> {
        tag.attributes.iter().collect()
    }

    #[test]
    fn emits_native_command_and_popover_contract() {
        let VNode::VList(list) = popover_view(&props()) else {
            panic!("popover should render trigger and surface siblings");
        };
        assert_eq!(list.len(), 2);

        let trigger = tag(&list[0]);
        let trigger_attrs = attrs(trigger);
        assert_eq!(trigger.tag(), "button");
        assert_eq!(trigger_attrs.get("command"), Some(&"toggle-popover"));
        assert_eq!(trigger_attrs.get("commandfor"), Some(&"deployment-options"));
        assert!(!trigger_attrs.contains_key("aria-label"));
        assert!(!trigger_attrs.contains_key("aria-describedby"));
        assert!(!trigger_attrs.contains_key("aria-haspopup"));
        assert!(!trigger_attrs.contains_key("aria-expanded"));
        assert!(!trigger_attrs.contains_key("interestfor"));
        assert_eq!(
            trigger_attrs.get("style"),
            Some(&"anchor-name: --popover-s-deployment-options;")
        );

        let surface = tag(&list[1]);
        let surface_attrs = attrs(surface);
        assert_eq!(surface.tag(), "div");
        assert_eq!(surface_attrs.get("id"), Some(&"deployment-options"));
        assert_eq!(surface_attrs.get("popover"), Some(&"auto"));
        assert_eq!(
            surface_attrs.get("class"),
            Some(&"popover popover-primary popover-bottom")
        );
        assert_eq!(
            surface_attrs.get("style"),
            Some(&"position-anchor: --popover-s-deployment-options;")
        );
    }

    #[test]
    fn supports_manual_surfaces_and_each_native_command() {
        let mut props = props();
        props.mode = PopoverMode::Manual;
        props.command = PopoverCommand::Show;

        let VNode::VList(list) = popover_view(&props) else {
            panic!("popover should render trigger and surface siblings");
        };
        assert_eq!(attrs(tag(&list[0])).get("command"), Some(&"show-popover"));
        assert_eq!(attrs(tag(&list[1])).get("popover"), Some(&"manual"));
        assert_eq!(PopoverCommand::Hide.as_str(), "hide-popover");
    }

    #[test]
    fn creates_css_safe_anchor_names() {
        assert_eq!(
            popover_anchor_name("row action:编辑"),
            "--popover-x-726f7720616374696f6e3ae7bc96e8be91"
        );
        assert_eq!(popover_anchor_name(""), "--popover-empty");
        assert_eq!(
            popover_anchor_name("deployment-options"),
            "--popover-s-deployment-options"
        );
    }

    #[test]
    fn action_trigger_has_independent_name_and_no_native_command() {
        let mut props = props();
        props.trigger_command_enabled = false;
        props.trigger_interest = true;
        props.trigger_label = "urgent".into();
        props.trigger_aria_label = Some("Filter notes by urgent".into());
        props.trigger_aria_describedby = Some(props.id.clone());
        props.trigger_onclick = Callback::from(|_: MouseEvent| ());

        let VNode::VList(list) = popover_view(&props) else {
            panic!("popover should render trigger and surface siblings");
        };
        let trigger = tag(&list[0]);
        let attributes = attrs(trigger);
        assert_eq!(attributes.get("type"), Some(&"button"));
        assert_eq!(attributes.get("interestfor"), Some(&"deployment-options"));
        assert_eq!(
            attributes.get("aria-label"),
            Some(&"Filter notes by urgent")
        );
        assert_eq!(
            attributes.get("aria-describedby"),
            Some(&"deployment-options")
        );
        assert_eq!(trigger.children(), Some(&html! { "urgent" }));
        assert!(!attributes.contains_key("command"));
        assert!(!attributes.contains_key("commandfor"));
        assert!(!attributes.contains_key("aria-haspopup"));
        assert!(!attributes.contains_key("aria-expanded"));

        // The callback is attached to the generated button, not the surface.
        let mut without_listeners = trigger.clone();
        without_listeners.set_listeners(Box::default());
        assert_ne!(*trigger, without_listeners);
        assert_eq!(attrs(tag(&list[1])).get("popover"), Some(&"auto"));
    }

    #[test]
    fn trigger_popup_and_expansion_aria_are_customizable_and_removable() {
        let mut props = props();
        props.trigger_aria_haspopup = Some("menu".into());
        props.trigger_aria_expanded = Some(false);

        let VNode::VList(list) = popover_view(&props) else {
            panic!("popover should render trigger and surface siblings");
        };
        let attributes = attrs(tag(&list[0]));
        assert_eq!(attributes.get("aria-haspopup"), Some(&"menu"));
        assert_eq!(attributes.get("aria-expanded"), Some(&"false"));
        assert_eq!(attributes.get("command"), Some(&"toggle-popover"));

        props.trigger_aria_haspopup = Some("dialog".into());
        props.trigger_aria_expanded = Some(true);
        let VNode::VList(list) = popover_view(&props) else {
            panic!("popover should render trigger and surface siblings");
        };
        assert_eq!(attrs(tag(&list[0])).get("aria-haspopup"), Some(&"dialog"));
        assert_eq!(attrs(tag(&list[0])).get("aria-expanded"), Some(&"true"));

        props.trigger_aria_haspopup = None;
        props.trigger_aria_expanded = None;
        let VNode::VList(list) = popover_view(&props) else {
            panic!("popover should render trigger and surface siblings");
        };
        assert!(!attrs(tag(&list[0])).contains_key("aria-haspopup"));
        assert!(!attrs(tag(&list[0])).contains_key("aria-expanded"));
    }
}
