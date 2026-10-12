use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::FilterGroup;

const API: &[ApiRow] = &[
    ApiRow { prop: "class", ty: "Classes", default: "empty", docs: "Additional Core or utility classes." },
    ApiRow { prop: "variant", ty: "Option<String>", default: "None", docs: "Core semantic color modifier: primary, secondary, tertiary, info, success, warning, or error." },
    ApiRow { prop: "children", ty: "Children", default: "empty", docs: "Direct label.chip children with filter-group-input native controls." },
    ApiRow { prop: "aria_label", ty: "AttrValue", default: "Filters", docs: "Accessible name for the selection group." },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::FilterGroup;

html! {
    <FilterGroup variant={Some("primary".to_owned())} aria_label="Status">
        <label class="chip"><input class="filter-group-input" type="radio" name="filter-status" value="all" checked={true} />{ "All" }</label>
        <label class="chip"><input class="filter-group-input" type="radio" name="filter-status" value="open" />{ "Open" }</label>
        <label class="chip"><input class="filter-group-input" type="radio" name="filter-status" value="closed" />{ "Closed" }</label>
    </FilterGroup>
}"#.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <FilterGroup variant={Some("primary".to_owned())} aria_label="Status">
                <label class="chip"><input class="filter-group-input" type="radio" name="filter-status" value="all" checked={true} />{ "All" }</label>
                <label class="chip"><input class="filter-group-input" type="radio" name="filter-status" value="open" />{ "Open" }</label>
                <label class="chip"><input class="filter-group-input" type="radio" name="filter-status" value="closed" />{ "Closed" }</label>
            </FilterGroup>
    }
}
