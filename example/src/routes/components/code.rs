use yew::prelude::*;
use yew_duskmoon::typography::TypographyLevel;
use yew_duskmoon::{Card, CodeBlock, MarkdownBody, Typography};

use super::palette::PALETTE;

#[function_component(CodeComponent)]
pub fn code_component() -> Html {
    html! {
        <div class="app">
            <div class="app-main component-main">
                <Typography level={TypographyLevel::H2}>{"Code / Markdown Components"}</Typography>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"CodeBlock Component"}</Typography> }} classes="component-card">
                    <div class="color-grid code-color-grid">
                        { for PALETTE.into_iter().map(|color| html! {
                            <CodeBlock class="code-block-compact">
                                <div class="code-header">
                                    <span class="code-title">{ color.label }</span>
                                    <span class="code-language">{ "rust" }</span>
                                </div>
                                <div class="code-content"><pre><code>{ format!("let token = \"{}\";", color.key) }</code></pre></div>
                            </CodeBlock>
                        }) }
                    </div>
                </Card>

                <Card title={html! { <Typography level={TypographyLevel::H4}>{"MarkdownBody Component"}</Typography> }} classes="component-card">
                    <div class="color-grid markdown-color-grid">
                        { for PALETTE.into_iter().map(|color| html! {
                            <MarkdownBody>
                                <h3>{ color.label }</h3>
                                <p>{"Token pair: "}<strong>{ color.key }</strong>{" and content."}</p>
                            </MarkdownBody>
                        }) }
                    </div>
                </Card>
            </div>
        </div>
    }
}
