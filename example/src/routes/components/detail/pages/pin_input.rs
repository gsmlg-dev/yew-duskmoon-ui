use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use crate::routes::components::palette::{variant, PaletteColor};
use yew::prelude::*;
use yew_duskmoon::PinInput;

const API: &[ApiRow] = &[
    ApiRow { prop: "class / variant", ty: "Classes / Option<String>", default: "empty / None", docs: "Core OTP root classes and semantic color." },
    ApiRow { prop: "children", ty: "Children", default: "empty", docs: "Optional legacy OTP composition; omission renders one complete native code input over decorative cells." },
    ApiRow { prop: "length", ty: "usize", default: "4", docs: "Code length clamped to the Core supported 1..8 cells." },
    ApiRow { prop: "id / name / aria_label", ty: "AttrValue", default: "empty / empty / Security PIN", docs: "Native input identity and accessible name." },
    ApiRow { prop: "value", ty: "Option<AttrValue>", default: "None", docs: "Optional controlled complete code value." },
    ApiRow { prop: "disabled / required", ty: "bool", default: "false", docs: "Native control state." },
    ApiRow { prop: "oninput / onchange", ty: "Callback", default: "noop", docs: "Native events from the single complete input, supporting paste and browser editing." },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, API, demo, color_variant)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::PinInput;

html! {
    <PinInput variant={Some("primary".to_owned())} length={4} name="code" aria_label="Security PIN" />
}"#.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <div class="otp-group">
            <label class="otp-label" for="pin_input-code">{ "Security PIN" }</label>
            <PinInput variant={Some("primary".to_owned())} length={4} id="pin_input-code" name="code" required={true} />
            <span class="otp-helper">{ "Enter or paste the complete 4-digit code." }</span>
        </div>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <div class="otp-group">
            <span class="otp-label">{ color.label }</span>
            <PinInput variant={variant(color)} length={4} aria_label={format!("{} code", color.label)} />
        </div>
    }
}
