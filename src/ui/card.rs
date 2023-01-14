use yew::prelude::*;

use stylist::css;
use stylist::yew::use_style;

/// Props for [`Card`]
#[derive(Properties, Clone, PartialEq)]
pub struct CardProps {
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    /// title part
    #[prop_or_default]
    pub title: Option<Html>,
    /// infor part
    #[prop_or_default]
    pub children: Children,
}

/// Card component
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let style = use_style(css!(
        r#"
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: flex-start;
    padding: 1.5rem;
    gap: 0.618rem;
    box-shadow: 0 0 #0000, 0 0 #0000, 0 1px 3px 0 #0000001a, 0 1px 2px -1px #0000001a;
    .head {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;
        height: 1.48rem;
    }
    .body {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: flex-start;
    }
    "#
    ));
    let owned_props = props.clone();

    html! {
        <div class={classes!(style, owned_props.classes)}>
            {
                match owned_props.title {
                    Some(_) => {
                        html! {
                            <div class="head">
                                { owned_props.title }
                            </div>
                        }
                    }
                    None => {
                        html! {}
                    }
                }
            }
            <div class="body">
            { for owned_props.children.iter() }
            </div>
        </div>
    }
}
