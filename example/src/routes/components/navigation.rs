use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Breadcrumbs, Card, Menu, Pagination, Stepper, Typography};

use super::palette::{variant, PALETTE};

#[function_component(NavigationComponent)]
pub fn navigation_component() -> Html {
    let current_page = use_state(|| 1);
    let select_page = {
        let current_page = current_page.clone();
        Callback::from(move |p: usize| current_page.set(p))
    };

    // TODO(upstream): duskmoon-dev/duskmoonui#74
    // WORKAROUND(upstream): duskmoon-dev/duskmoonui#74 - keep semantic current-page state with primary until alternate colors are fixed.
    let pagination_colors = PALETTE.into_iter().filter(|color| color.key == "primary");

    html! {
        <div class="app" id="navigation">
            <div class="app-main component-main">
                <Typography level={TypographyLevel::H2}>{"Navigation Components"}</Typography>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Breadcrumbs"}</Typography> }} classes="component-card">
                    <div class="color-grid compact-color-grid">
                        { for PALETTE.into_iter().filter(|color| matches!(color.key, "primary" | "secondary" | "tertiary")).map(|color| html! {
                            <Breadcrumbs variant={variant(color)}>
                                <a href="#navigation" class="breadcrumb-link">{ color.label }</a>
                                <span class="breadcrumb-separator" aria-hidden="true"></span>
                                <span class="breadcrumb-item breadcrumb-item-active" aria-current="page">{"Navigation"}</span>
                            </Breadcrumbs>
                        }) }
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"Menu"}</Typography> }} classes="component-card">
                    <Menu class="menu-vertical">
                        <div class="menu-title">{"Navigation"}</div>
                        <a href="#navigation" class="menu-item" aria-current="page">{"Overview"}</a>
                        <a href="#pagination" class="menu-item">{"Pagination"}</a>
                        <a href="#stepper" class="menu-item">{"Stepper"}</a>
                        <button type="button" class="menu-item" disabled={true}>{"Archived"}</button>
                    </Menu>
                </Card>

                <div id="pagination">
                    <Card title={html! { <Typography level={TypographyLevel::H4}>{"Pagination"}</Typography> }} classes="component-card">
                        <div class="color-grid pagination-color-grid">
                            { for pagination_colors.map(|color| html! {
                                <div class="pagination-color-cell">
                                    <span>{ color.label }</span>
                                    <Pagination>
                                        {
                                            for (1..=3).map(|p| {
                                                let select = select_page.clone();
                                                let active = p == *current_page;
                                                let btn_class = classes!("pagination-item", active.then(|| format!("pagination-item-active-{}", color.key)));
                                                html! {
                                                    <button type="button" onclick={move |_| select.emit(p)} class={btn_class}
                                                        aria-current={active.then_some("page")} aria-label={format!("Page {p}")}>
                                                        { p }
                                                    </button>
                                                }
                                            })
                                        }
                                    </Pagination>
                                </div>
                            }) }
                        </div>
                    </Card>
                </div>

                <div id="stepper">
                    <Card title={html! { <Typography level={TypographyLevel::H4}>{"Stepper"}</Typography> }} classes="component-card">
                        <div class="stepper-palette">
                            { for PALETTE.into_iter().filter(|color| matches!(color.key, "primary" | "secondary" | "tertiary")).map(|color| html! {
                                <Stepper variant={if color.key == "primary" { None } else { variant(color) }}>
                                    <div class="stepper-step stepper-step-completed">
                                        <button type="button" class="stepper-step-button">
                                            <span class="stepper-step-icon">{"1"}</span>
                                            <span class="stepper-step-label">{"Account"}</span>
                                        </button>
                                        <div class="stepper-step-connector"></div>
                                    </div>
                                    <div class="stepper-step stepper-step-active" aria-current="step">
                                        <button type="button" class="stepper-step-button">
                                            <span class="stepper-step-icon">{"2"}</span>
                                            <span class="stepper-step-label">{color.label}</span>
                                        </button>
                                        <div class="stepper-step-connector"></div>
                                    </div>
                                    <div class="stepper-step stepper-step-disabled">
                                        <button type="button" class="stepper-step-button" disabled={true}>
                                            <span class="stepper-step-icon">{"3"}</span>
                                            <span class="stepper-step-label">{"Confirm"}</span>
                                        </button>
                                    </div>
                                </Stepper>
                            }) }
                        </div>
                    </Card>
                </div>
            </div>
        </div>
    }
}
