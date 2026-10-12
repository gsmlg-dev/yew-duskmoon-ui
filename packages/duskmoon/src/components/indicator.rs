use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct IndicatorProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Indicator)]
pub fn indicator(props: &IndicatorProps) -> Html {
    let classes = classes!("indicator", props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
