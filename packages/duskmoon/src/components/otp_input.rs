use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct OtpInputProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or(6)]
    pub length: usize,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or(AttrValue::from("Verification code"))]
    pub aria_label: AttrValue,
    #[prop_or(AttrValue::from("text"))]
    pub input_type: AttrValue,
    #[prop_or(AttrValue::from("one-time-code"))]
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

#[function_component(OtpInput)]
pub fn otp_input(props: &OtpInputProps) -> Html {
    let mut classes = classes!("otp-input");
    if let Some(variant) = &props.variant {
        classes.push(format!("otp-input-{}", variant));
    }
    classes.push(props.class.clone());

    let length = normalized_length(props.length);
    html! {
        <div class={classes}>
            { if props.children.is_empty() {
                html! {
                    <>
                        { for (0..length).map(|_| html! { <span aria-hidden="true" /> }) }
                        <input class="otp-code" type={props.input_type.clone()} inputmode="numeric"
                            id={props.id.clone()} name={props.name.clone()} value={props.value.clone()}
                            aria-label={props.aria_label.clone()} autocomplete={props.autocomplete.clone()}
                            maxlength={length.to_string()} pattern={format!("[0-9]{{{}}}", length)}
                            disabled={props.disabled} required={props.required}
                            oninput={props.oninput.clone()} onchange={props.onchange.clone()} />
                    </>
                }
            } else { html! { <>{ for props.children.iter() }</> } } }
        </div>
    }
}

fn normalized_length(length: usize) -> usize {
    length.clamp(1, 8)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn code_length_matches_the_supported_decorative_cell_count() {
        assert_eq!(normalized_length(0), 1);
        assert_eq!(normalized_length(4), 4);
        assert_eq!(normalized_length(12), 8);
    }
}
