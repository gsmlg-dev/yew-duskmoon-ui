use crate::routes::components::catalog::ComponentSpec;
use crate::routes::components::detail::page::{ApiRow, ComponentPage};
use yew::prelude::*;
use yew_duskmoon::CodeBlock;

const CODE_BLOCK_API: &[ApiRow] = &[
    ApiRow {
        prop: "class",
        ty: "Classes",
        default: "empty",
        docs: "Extra CSS classes appended to the code block root.",
    },
    ApiRow {
        prop: "children",
        ty: "Children",
        default: "empty",
        docs: "Header, language badge, copy button, and pre/code content.",
    },
    ApiRow {
        prop: "variant",
        ty: "Option<String>",
        default: "None",
        docs: "Structural modifier escape hatch; Core keeps the code surface neutral.",
    },
];

pub fn page(spec: &'static ComponentSpec) -> ComponentPage {
    ComponentPage::without_color_variants(spec, usage, CODE_BLOCK_API, demo)
}

fn usage(_: &ComponentSpec) -> String {
    "use yew_duskmoon::CodeBlock;\n\nhtml! {\n    <CodeBlock>\n        <div class=\"code-header\">\n            <span class=\"code-title\">{ \"example.rs\" }</span>\n            <span class=\"code-language\">{ \"rust\" }</span>\n        </div>\n        <div class=\"code-content\">\n            <pre><code>{ \"html! { <Button>{ \\\"Save\\\" }</Button> }\" }</code></pre>\n        </div>\n    </CodeBlock>\n}".to_owned()
}

fn demo(_: &ComponentSpec) -> Html {
    html! {
        <CodeBlock class="code-block-compact">
            <div class="code-header">
                <span class="code-title">{ "example.rs" }</span>
                <span class="code-language">{ "rust" }</span>
                <button type="button" class="copy-button">
                    <span class="copy-text">{ "Copy" }</span>
                </button>
            </div>
            <div class="code-content">
                <pre><code>{ "html! {\n    <Button variant={Some(\"primary\".to_owned())}>{ \"Save\" }</Button>\n}" }</code></pre>
            </div>
        </CodeBlock>
    }
}
