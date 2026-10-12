use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::Diff;

const EXTRA_API: &[ApiRow] = &[ApiRow {
    prop: "position",
    ty: "u8",
    default: "50",
    docs: "Revealed percentage, clamped to 100 and passed to Core's --diff-position variable.",
}];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
        .with_additional_api("Additional props", EXTRA_API)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Diff;

html! {
    <Diff position={60}><div class="diff-before bg-primary-container p-8"><span class="diff-label">{ "Before" }</span><p>{ "Previous layout" }</p></div><div class="diff-after bg-secondary-container p-8"><span class="diff-label">{ "After" }</span><p>{ "Updated layout" }</p></div></Diff>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Diff position={60}><div class="diff-before bg-primary-container p-8"><span class="diff-label">{ "Before" }</span><p>{ "Previous layout" }</p></div><div class="diff-after bg-secondary-container p-8"><span class="diff-label">{ "After" }</span><p>{ "Updated layout" }</p></div></Diff>
    }
}
