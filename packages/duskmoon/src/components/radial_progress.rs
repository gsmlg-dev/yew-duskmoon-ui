use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct RadialProgressProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub value: f64,
    #[prop_or(AttrValue::from("Progress"))]
    pub aria_label: AttrValue,
}

#[function_component(RadialProgress)]
pub fn radial_progress(props: &RadialProgressProps) -> Html {
    let mut classes = classes!("radial-progress");
    if let Some(variant) = &props.variant {
        classes.push(format!("radial-progress-{}", variant));
    }
    classes.push(props.class.clone());
    let value = normalized_value(props.value);
    html! {
        <div class={classes} style={format!("--radial-progress-value: {}", value)} role="progressbar"
            aria-label={props.aria_label.clone()} aria-valuemin="0" aria-valuemax="100" aria-valuenow={value.to_string()}>
            <span class="relative">{
                if props.children.is_empty() { html! { format!("{}%", value) } }
                else { html! { <>{ for props.children.iter() }</> } }
            }</span>
        </div>
    }
}

fn normalized_value(value: f64) -> f64 {
    if value.is_finite() {
        value.clamp(0.0, 100.0)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalizes_visual_and_accessible_progress_together() {
        assert_eq!(normalized_value(-2.0), 0.0);
        assert_eq!(normalized_value(120.0), 100.0);
        assert_eq!(normalized_value(68.0), 68.0);
        assert_eq!(normalized_value(f64::NAN), 0.0);
    }
}
