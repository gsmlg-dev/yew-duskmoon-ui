use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::{Checkbox, Form, FormGroup, Input};

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo).with_additional_api(
        "Native form",
        &[ApiRow {
            prop: "onsubmit",
            ty: "Callback<SubmitEvent>",
            default: "noop",
            docs: "Native form submission callback; applications can prevent default navigation.",
        }],
    )
}

fn usage(_: &ComponentSpec) -> String {
    r#"use yew_duskmoon::{Form, FormGroup, Input};

html! {
    <Form onsubmit={Callback::from(|event: SubmitEvent| event.prevent_default())}>
        <FormGroup>
            <label class="form-label form-label-required" for="profile-email">{ "Email" }</label>
            <Input id="profile-email" name="email" input_type="email" class="input-bordered" required={true} />
            <span class="form-hint">{ "Use your work email." }</span>
        </FormGroup>
        <div class="form-actions">
            <button class="btn btn-primary" type="submit">{ "Save" }</button>
        </div>
    </Form>
}"#
    .to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <Form onsubmit={Callback::from(|event: SubmitEvent| event.prevent_default())} class="component-detail-demo-control">
            <div class="form-section">
                <h3 class="form-section-title">{ "Profile" }</h3>
                <p class="form-section-description">{ "A compact form using labels, inputs, validation copy, and actions." }</p>
            </div>
            <div class="form-grid form-grid-2">
                <FormGroup>
                    <label class="form-label form-label-required" for="profile-name">{ "Full name" }</label>
                    <Input id="profile-name" name="name" class="input-bordered" placeholder="Ada Lovelace" />
                    <span class="form-hint">{ "Shown on invoices and activity." }</span>
                </FormGroup>
                <FormGroup variant={Some("success".to_owned())}>
                    <label class="form-label" for="profile-email">{ "Email" }</label>
                    <Input id="profile-email" name="email" input_type="email" variant={Some("success".to_owned())} class="input-bordered" placeholder="ada@example.com" />
                    <span class="form-hint">{ "Verified." }</span>
                </FormGroup>
            </div>
            <Checkbox variant={Some("primary".to_owned())} class="checkbox-group">
                <label class="checkbox-label">
                    <input class="checkbox checkbox-primary" type="checkbox" checked={true} />
                    <span>{ "Send product updates" }</span>
                </label>
            </Checkbox>
            <div class="form-actions form-actions-right">
                <button class="btn btn-secondary" type="button">{ "Cancel" }</button>
                <button class="btn btn-primary" type="submit">{ "Save changes" }</button>
            </div>
        </Form>
    }
}
