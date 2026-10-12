use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::{Badge, BadgeSize, List};

const LIST_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the list root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "List item rows, leading/trailing slots, dividers, and subheaders.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs:
            "Structural modifier escape hatch; active colors belong on list-item-active-* children.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, LIST_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::List;\n\nhtml! {\n    <List class=\"list-bordered\">\n        <div class=\"list-item list-item-two-line list-item-active list-item-active-primary\">\n            <div class=\"list-item-content\">\n                <span class=\"list-item-text\">{ \"Inbox\" }</span>\n                <span class=\"list-item-secondary\">{ \"12 unread messages\" }</span>\n            </div>\n        </div>\n    </List>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <List class="component-detail-list-demo list-bordered">
            <div class="list-item list-item-two-line list-item-interactive list-item-active">
                <div class="list-item-content">
                    <span class="list-item-text">{ "Inbox" }</span>
                    <span class="list-item-secondary">{ "12 unread messages" }</span>
                </div>
                <div class="list-item-trailing">
                    <Badge size={BadgeSize::Small}>{ "12" }</Badge>
                </div>
            </div>
            <div class="list-item list-item-two-line list-item-interactive">
                <div class="list-item-content">
                    <span class="list-item-text">{ "Build queue" }</span>
                    <span class="list-item-secondary">{ "3 jobs waiting for review" }</span>
                </div>
            </div>
            <div class="list-item list-item-disabled">
                <div class="list-item-content">
                    <span class="list-item-text">{ "Archived" }</span>
                </div>
            </div>
        </List>
    }
}
