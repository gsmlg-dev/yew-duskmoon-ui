use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use crate::routes::components::palette::PaletteColor;
use yew::prelude::*;
use yew_duskmoon::Timeline;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra timeline classes for size, orientation, and density.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "timeline-item children with timeline-marker and timeline-content slots. Colors belong on item or marker slots.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Root structural modifier, such as compact or horizontal.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Timeline;

html! {
    <Timeline>
        <div class="timeline-item timeline-item-primary completed">
            <div class="timeline-marker"><span class="timeline-marker-icon">{ "OK" }</span></div>
            <div class="timeline-content">
                <time class="timeline-time" datetime="2026-07-07">{ "09:00" }</time>
                <h3 class="timeline-title">{ "Release cut" }</h3>
                <p class="timeline-description">{ "Artifacts were built and signed." }</p>
            </div>
        </div>
    </Timeline>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Timeline class="component-detail-demo-control timeline-progress">
            <div class="timeline-item completed">
                <div class="timeline-marker"><span class="timeline-marker-icon">{ "OK" }</span></div>
                <div class="timeline-content">
                    <time class="timeline-time" datetime="2026-07-07T09:00:00">{ "09:00" }</time>
                    <h3 class="timeline-title">{ "Build completed" }</h3>
                    <p class="timeline-description">{ "Package artifacts passed verification." }</p>
                </div>
            </div>
            <div class="timeline-item active">
                <div class="timeline-marker"><span class="timeline-marker-dot"></span></div>
                <div class="timeline-content">
                    <time class="timeline-time" datetime="2026-07-07T10:15:00">{ "10:15" }</time>
                    <h3 class="timeline-title">{ "Deploying" }</h3>
                    <p class="timeline-description">{ "The release is rolling through production." }</p>
                </div>
            </div>
            <div class="timeline-item timeline-item-warning">
                <div class="timeline-marker"><span class="timeline-marker-dot"></span></div>
                <div class="timeline-content">
                    <time class="timeline-time" datetime="2026-07-07T11:00:00">{ "11:00" }</time>
                    <h3 class="timeline-title">{ "Observe metrics" }</h3>
                    <p class="timeline-description">{ "Watch latency and error budgets after rollout." }</p>
                </div>
            </div>
        </Timeline>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    if color.key == "info" {
        return Html::default();
    }
    let (item_class, marker_class, modifier) = match color.key {
        "primary" | "secondary" | "success" | "warning" | "error" => (
            classes!("timeline-item", format!("timeline-item-{}", color.key)),
            classes!("timeline-marker"),
            format!("timeline-item-{}", color.key),
        ),
        _ => (
            classes!("timeline-item"),
            classes!("timeline-marker", format!("timeline-marker-{}", color.key)),
            format!("timeline-marker-{}", color.key),
        ),
    };
    html! {
        <Timeline class="component-detail-color-demo">
            <div class={item_class}>
                <div class={marker_class}><span class="timeline-marker-dot"></span></div>
                <div class="timeline-content">
                    <strong class="timeline-title">{ color.label }</strong>
                    <span class="timeline-time">{ modifier }</span>
                </div>
            </div>
        </Timeline>
    }
}
