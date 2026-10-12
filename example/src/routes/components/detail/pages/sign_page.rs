use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ComponentPage, STANDARD_API};
use yew::prelude::*;
use yew_duskmoon::SignPage;

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, STANDARD_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    r##"use yew_duskmoon::SignPage;

html! {
    <SignPage class="[--sign-page-min-height:20rem]"><div class="sign-page-frame"><aside class="sign-page-aside"><h3>{ "DuskMoon" }</h3><p>{ "A shared design system for your application." }</p></aside><main class="sign-page-main"><div class="sign-page-content"><h3>{ "Sign in" }</h3><label for="sign-demo-email">{ "Email" }</label><input class="input" id="sign-demo-email" type="email" autocomplete="email" /><button class="btn btn-primary mt-4" type="button">{ "Continue" }</button></div></main></div></SignPage>
}"##.to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <SignPage class="[--sign-page-min-height:20rem]"><div class="sign-page-frame"><aside class="sign-page-aside"><h3>{ "DuskMoon" }</h3><p>{ "A shared design system for your application." }</p></aside><section class="sign-page-main"><div class="sign-page-content"><h3>{ "Sign in" }</h3><label for="sign-demo-email">{ "Email" }</label><input class="input" id="sign-demo-email" type="email" autocomplete="email" /><button class="btn btn-primary mt-4" type="button">{ "Continue" }</button></div></section></div></SignPage>
    }
}
