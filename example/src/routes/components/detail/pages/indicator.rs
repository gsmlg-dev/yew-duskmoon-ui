use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::Indicator;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra classes on the inline anchor container.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Anchor content and indicator-item slots. Put positioning classes such as indicator-bottom and indicator-start on each indicator-item.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::Indicator;

html! {
    <Indicator><span class="indicator-item indicator-top indicator-end badge badge-primary">{ "3" }</span><button class="btn btn-outlined" type="button">{ "Inbox" }</button></Indicator>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="flex flex-wrap gap-8">
            <Indicator><span class="indicator-item indicator-top indicator-end badge badge-primary">{ "3" }</span><button class="btn btn-outlined" type="button">{ "Inbox" }</button></Indicator>
            <Indicator><span class="indicator-item indicator-bottom indicator-start badge badge-secondary">{ "2" }</span><button class="btn btn-outlined" type="button">{ "Drafts" }</button></Indicator>
        </div>
    }
}
