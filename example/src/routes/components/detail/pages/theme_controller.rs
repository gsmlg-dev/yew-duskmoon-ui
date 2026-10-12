use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use crate::states::config::ConfigStore;
use yew::prelude::*;
use yew_duskmoon::ThemeController;
use yewdux::prelude::*;

const API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Additional Core classes, including theme-controller-sm and theme-controller-lg.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Native theme-controller-item radio inputs followed by theme-controller-label labels. Applications own theme updates.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Core modifiers such as sm, lg, or icon; this component has no semantic color variants.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::ThemeController;

html! {
    <ThemeController>
        <input id="theme-auto" class="theme-controller-item" type="radio" name="theme" value="auto" />
        <label for="theme-auto" class="theme-controller-label">{ "Auto" }</label>
        <input id="theme-light" class="theme-controller-item" type="radio" name="theme" value="sunshine" />
        <label for="theme-light" class="theme-controller-label">{ "Light" }</label>
        <input id="theme-dark" class="theme-controller-item" type="radio" name="theme" value="moonlight" />
        <label for="theme-dark" class="theme-controller-label">{ "Dark" }</label>
    </ThemeController>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! { <ThemeDemo /> }
}

#[function_component(ThemeDemo)]
fn theme_demo() -> Html {
    let (state, dispatch) = use_store::<ConfigStore>();

    html! {
        <fieldset class="form-group">
            <legend class="form-label">{ "Theme" }</legend>
            <ThemeController>
                { for [("auto", "Auto"), ("sunshine", "Light"), ("duskmoon", "Dark")].into_iter().map(|(mode, label)| {
                    let onchange = dispatch.reduce_mut_callback(move |config| config.theme_mode = mode.to_owned());
                    let id = format!("detail-theme-{}", mode);
                    html! {
                        <>
                            <input id={id.clone()} class="theme-controller-item" type="radio" name="detail-theme"
                                value={mode} checked={state.theme_mode == mode} onchange={onchange} />
                            <label for={id} class="theme-controller-label">{ label }</label>
                        </>
                    }
                }) }
            </ThemeController>
            <span class="helper-text">{ "Select a theme for this application. Auto follows your system preference." }</span>
        </fieldset>
    }
}
