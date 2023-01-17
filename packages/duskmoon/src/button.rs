use yew::prelude::*;

use stylist::css;
use stylist::yew::use_style;

/// Props for [`Button`]
#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    /// CSS classes to add to the anchor element (optional).
    #[prop_or_default]
    pub classes: Classes,
    /// infor part
    #[prop_or_default]
    pub children: Children,
}

/// Button component
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let style = use_style(css!(
        r#"
        line-height: 1.5715;
        position: relative;
        display: inline-flex;
        justify-content: center;
        align-items: center;
        font-weight: 400;
        white-space: nowrap;
        background-image: none;
        border: 1px solid transparent;
        box-shadow: 0 2px 0 rgba(0, 0, 0, 0.015);
        cursor: pointer;
        transition: all 0.3s cubic-bezier(0.645, 0.045, 0.355, 1);
        user-select: none;
        touch-action: manipulation;
        height: 2.25rem;
        padding: 0.3rem 1.1rem;
        font-size: 1rem;
        border-radius: 0.2rem;
        color: rgba(0, 0, 0, 0.85);
        border-color: var(--btn-border, #d9d9d9);
        background: #fff;
        text-decoration: none;

        & icon-spin {
            margin-left: -0.4rem;
            margin-right: 0.4rem;
        }
        &.btn-primary {
            color: #fff;
            border-color: var(--btn-primary, #1890ff);
            background: var(--btn-primary, #1890ff);
        }
        &.btn-dashed {
            border: 1px dashed transparent;
            background: transparent;
            border-color: rgba(0, 0, 0, 0.85);
        }
        &.btn-danger {
            color: rgb(255, 255, 255);
            border-color: var(--btn-danger, #ff4d4f);
            background: var(--btn-danger, #ff4d4f);
            text-shadow: rgb(0 0 0 / 12%) 0px -1px 0px;
            box-shadow: rgb(0 0 0 / 4%) 0px 2px 0px;
        }
        &.btn-link,
        &.btn-text {
            color: rgba(0, 0, 0, 0.85);
            border: none;
            background: transparent;
            text-shadow: rgb(0 0 0 / 12%) 0px -1px 0px;
            box-shadow: none;
        }
        &.btn-link {
            color: var(--btn-link, #1890ff);
        }
        &.btn-round {
            height: 2.25rem;
            padding: 0.3rem 1.1rem;
            font-size: 1rem;
            border-radius: 2.25rem;
        }
        &.btn-circle {
            height: 2.25rem;
            width: 2.25rem;
            padding: 0.3rem 1.1rem;
            font-size: 1rem;
            border-radius: 50%;
        }
        &.btn-circle icon-spin {
            position: absolute;
            margin: 0;
        }
        &.btn-block {
            height: 2.25rem;
            width: 100%;
            padding: 0.3rem 1.1rem;
            font-size: 1rem;
        }
        &:hover {
            color: var(--btn-hover, #40a9ff);
            border-color: var(--btn-hover, #40a9ff);
        }
        &.btn-primary:hover {
            color: #fff;
            background: var(--btn-primary-hover, #40a9ff);
            border-color: var(--btn-primary-hover, #40a9ff);
        }
        &.btn-danger:hover {
            color: #fff;
            background: var(--btn-danger-hover, #ff7875);
            border-color: var(--btn-danger-hover, #ff7875);
        }
        &[disabled] {
            cursor: not-allowed;
        }
        &[disabled].btn-loading {
            cursor: progress;
        }
        &[disabled]:not(.btn-loading),
        &[disabled].disabled {
            color: rgba(0, 0, 0, 0.25);
            background: var(--btn-disabled, #f5f5f5);
            broder-color: var(--btn-border-disabled, #d9d9d9);
            border: none;
        }
    "#
    ));
    let owned_props = props.clone();

    html! {
        <button class={ classes!(style, owned_props.classes) }>
        { for owned_props.children.iter() }
        </button>
    }
}
