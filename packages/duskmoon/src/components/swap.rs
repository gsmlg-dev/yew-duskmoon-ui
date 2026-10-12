use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SwapProps {
    /// Accessible name independent of the two visual indicators.
    pub label: AttrValue,
    pub on: Html,
    pub off: Html,
    #[prop_or_default]
    pub pressed: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub on_change: Callback<bool>,
}

/// Controlled toggle button. `on` and `off` are non-interactive indicators.
#[function_component(Swap)]
pub fn swap(props: &SwapProps) -> Html {
    swap_view(props)
}

fn swap_view(props: &SwapProps) -> Html {
    let on_change = props.on_change.clone();
    let pressed = props.pressed;
    let disabled = props.disabled;
    let onclick = Callback::from(move |_| {
        if !disabled {
            on_change.emit(!pressed);
        }
    });
    html! {
        <button type="button" class={classes!("swap", props.class.clone())} aria-label={props.label.clone()}
            aria-pressed={props.pressed.to_string()} disabled={props.disabled} {onclick}>
            <span class="swap-on" aria-hidden="true">{props.on.clone()}</span>
            <span class="swap-off" aria-hidden="true">{props.off.clone()}</span>
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew::virtual_dom::VNode;

    #[test]
    fn semantic_pressed_state_drives_core_indicators() {
        let mut props = SwapProps {
            label: "Mute".into(),
            on: html! {"Muted"},
            off: html! {"Sound on"},
            pressed: false,
            disabled: false,
            class: Classes::new(),
            on_change: Callback::default(),
        };
        for pressed in [false, true] {
            props.pressed = pressed;
            let VNode::VTag(button) = swap_view(&props) else {
                panic!("expected button");
            };
            assert_eq!(button.tag(), "button");
            assert!(button
                .attributes
                .iter()
                .any(|(key, value)| key == "aria-pressed" && value == pressed.to_string()));
            let Some(VNode::VList(children)) = button.children() else {
                panic!("expected indicators");
            };
            for child in children.iter() {
                let VNode::VTag(indicator) = child else {
                    panic!("expected indicator");
                };
                assert!(indicator
                    .attributes
                    .iter()
                    .any(|(key, value)| key == "aria-hidden" && value == "true"));
            }
        }
    }
}
