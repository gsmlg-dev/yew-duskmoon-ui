use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, STANDARD_API};
use crate::routes::components::palette::{variant, PaletteColor};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_duskmoon::Checkbox;

const NATIVE_API: &[ApiRow] = &[
    ApiRow {
        prop: "id / name / value",
        ty: "AttrValue",
        default: "empty / empty / on",
        docs: "Native control identity and submitted value in direct mode.",
    },
    ApiRow {
        prop: "checked",
        ty: "Option<bool>",
        default: "None",
        docs: "Controlled state when set; omission retains native browser toggling.",
    },
    ApiRow {
        prop: "disabled / required",
        ty: "bool",
        default: "false",
        docs: "Native control state in direct mode.",
    },
    ApiRow {
        prop: "onchange",
        ty: "Callback<Event>",
        default: "noop",
        docs: "Native change events in direct mode.",
    },
    ApiRow {
        prop: "aria_label",
        ty: "AttrValue",
        default: "empty",
        docs: "Accessible name for direct native control rendering.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::new(spec, usage, STANDARD_API, demo, color_variant)
        .with_additional_api("Native control props", NATIVE_API)
}

#[function_component(IndeterminateCheckbox)]
fn indeterminate_checkbox() -> Html {
    let input_ref = use_node_ref();

    {
        let input_ref = input_ref.clone();
        use_effect_with((), move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.set_indeterminate(true);
            }
        });
    }

    html! {
        <input ref={input_ref} class="checkbox checkbox-info" type="checkbox" />
    }
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::Checkbox;

html! {
    <Checkbox variant={Some("primary".to_owned())} class="checkbox-group">
        <label class="checkbox-label">
            <input class="checkbox checkbox-primary" type="checkbox" checked={true} />
            <span>{ "Accept terms and conditions" }</span>
        </label>
    </Checkbox>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Checkbox variant={Some("primary".to_owned())} class="component-detail-demo-control checkbox-group">
            <label class="checkbox-label">
                <input class="checkbox checkbox-primary" type="checkbox" checked={true} />
                <span>{ "Email release notes" }</span>
            </label>
            <label class="checkbox-label">
                <input class="checkbox checkbox-secondary" type="checkbox" />
                <span>{ "Invite me to beta programs" }</span>
            </label>
            <label class="checkbox-label">
                <IndeterminateCheckbox />
                <span>{ "Select all release channels" }</span>
            </label>
            <label class="checkbox-label">
                <input class="checkbox checkbox-success" type="checkbox" checked={true} disabled={true} />
                <span>{ "Security alerts required" }</span>
            </label>
        </Checkbox>
    }
}

fn color_variant(color: PaletteColor) -> Html {
    html! {
        <Checkbox variant={variant(color)} class="component-detail-color-demo">
            <label class="checkbox-label">
                <input class={classes!("checkbox", format!("checkbox-{}", color.key))} type="checkbox" checked={true} />
                <span>{ color.label }</span>
            </label>
        </Checkbox>
    }
}
