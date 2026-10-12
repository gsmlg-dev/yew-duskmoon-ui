use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DiffProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
    /// Percentage of the before surface revealed by Core's clipping contract.
    #[prop_or(50)]
    pub position: u8,
}

#[function_component(Diff)]
pub fn diff(props: &DiffProps) -> Html {
    let mut classes = classes!("diff");
    if let Some(variant) = &props.variant {
        classes.push(format!("diff-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <figure class={classes} style={position_style(props.position)}>
            { for props.children.iter() }
        </figure>
    }
}

fn position_style(position: u8) -> String {
    format!("--diff-position: {}%;", position.min(100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constrains_the_revealed_surface_to_a_valid_percentage() {
        assert_eq!(position_style(0), "--diff-position: 0%;");
        assert_eq!(position_style(65), "--diff-position: 65%;");
        assert_eq!(position_style(255), "--diff-position: 100%;");
    }
}
