use serde::Serialize;
use yew::prelude::*;

use yew_router::components::Link as YewLink;
use yew_router::Routable;

use stylist::css;
use stylist::yew::use_style;

/// Props for [`Link`]
#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps<R, Q = ()>
where
    R: Routable,
    Q: Clone + PartialEq + Serialize,
{
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    /// Route that will be pushed when the anchor is clicked.
    pub to: R,
    /// Route query data
    #[prop_or_default]
    pub query: Option<Q>,
    #[prop_or_default]
    pub disabled: bool,
    /// [`NodeRef`](yew::html::NodeRef) for the `<a>` element.
    #[prop_or_default]
    pub anchor_ref: NodeRef,
    #[prop_or_default]
    pub children: Children,
}

/// A wrapper around `<Link<R,Q>` tag to be used with [`Router`](crate::Router)
#[function_component(Link)]
pub fn link<R, Q = ()>(props: &LinkProps<R, Q>) -> Html
where
    R: Routable + 'static,
    Q: Clone + PartialEq + Serialize + 'static,
{
    let style = use_style(css!(
        r#"
        min-width: 2em;
        padding: 0.4em 0.8em;
        border: none;
        text-align: center;
        cursor: pointer;
        transition: all 0.3s;
        text-decoration: none;
        color: #44c2f4;

        &:hover {
            text-shadow: 0.3px 0.3px #aaa;
        }
    "#
    ));

    let np = props.clone();

    html! {
        <YewLink<R,Q> classes={classes!(style, np.classes)}
            to={np.to}
            query={np.query}
            disabled={np.disabled}
            anchor_ref={np.anchor_ref}
        >
            { np.children }
        </YewLink<R,Q>>
    }
}
