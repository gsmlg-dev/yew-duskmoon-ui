use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use crate::routes::components::ComponentsRoute;
use yew::prelude::*;
use yew_duskmoon::Link;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "to",
        ty: "R: Routable",
        default: "required",
        docs: "Typed destination passed to Yew Router.",
    },
    ApiRow {
        prop: "classes",
        ty: "Classes",
        default: "empty",
        docs: "Additional classes on the native link.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Core link modifier such as primary or hover.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"html! {
    <Link<ComponentsRoute> to={ComponentsRoute::ComponentsRoot} variant={Some("primary".to_owned())}>
        { "Component catalog" }
    </Link<ComponentsRoute>>
}"#.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Link<ComponentsRoute> to={ComponentsRoute::ComponentsRoot} variant={Some("primary".to_owned())}>
            { "Component catalog" }
        </Link<ComponentsRoute>>
    }
}
