use super::otp_input::OtpInput;
use yew::prelude::*;

/// Compatibility alias for OTP composition. Use `.otp-code` or `.otp-input-field` children.
#[derive(Properties, Clone, PartialEq)]
pub struct PinInputProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or(4)]
    pub length: usize,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or(AttrValue::from("Security PIN"))]
    pub aria_label: AttrValue,
    #[prop_or(AttrValue::from("off"))]
    pub autocomplete: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(PinInput)]
pub fn pin_input(props: &PinInputProps) -> Html {
    html! {
        <OtpInput class={props.class.clone()} variant={props.variant.clone()} length={props.length}
            id={props.id.clone()} name={props.name.clone()} value={props.value.clone()}
            aria_label={props.aria_label.clone()} input_type="password" autocomplete={props.autocomplete.clone()}
            disabled={props.disabled} required={props.required} oninput={props.oninput.clone()}
            onchange={props.onchange.clone()}>
            { for props.children.iter() }
        </OtpInput>
    }
}
