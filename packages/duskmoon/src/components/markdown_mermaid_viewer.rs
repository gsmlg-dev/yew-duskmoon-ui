use std::sync::atomic::{AtomicUsize, Ordering};

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, FocusOptions, HtmlDialogElement, HtmlElement};
use yew::prelude::*;

// Cloned arrow markers need their own IDs so they resolve inside the viewer.
static NEXT_VIEWER_ID: AtomicUsize = AtomicUsize::new(0);

#[hook]
pub(super) fn use_mermaid_viewer(rendered: AttrValue) -> (NodeRef, Callback<MouseEvent>, Html) {
    let markdown_ref = use_node_ref();
    let dialog_ref = use_node_ref();
    let body_ref = use_node_ref();

    {
        let markdown_ref = markdown_ref.clone();
        let dialog_ref = dialog_ref.clone();
        let body_ref = body_ref.clone();
        use_effect_with(rendered, move |_| {
            if let Some(dialog) = dialog_ref.cast::<HtmlDialogElement>() {
                dialog.close();
            }
            if let Some(body) = body_ref.cast::<Element>() {
                body.set_inner_html("");
            }
            if let Some(markdown) = markdown_ref.cast::<Element>() {
                if let Err(error) = add_zoom_buttons(&markdown) {
                    log::warn!("Could not add Mermaid zoom buttons: {error:?}");
                }
            }
        });
    }

    let onclick = {
        let dialog_ref = dialog_ref.clone();
        let body_ref = body_ref.clone();
        Callback::from(move |event: MouseEvent| {
            let Some(target) = event
                .target()
                .and_then(|target| target.dyn_into::<Element>().ok())
            else {
                return;
            };
            let Ok(Some(preview)) = target.closest(".dm-mermaid-preview") else {
                return;
            };
            let (Some(dialog), Some(body)) = (
                dialog_ref.cast::<HtmlDialogElement>(),
                body_ref.cast::<Element>(),
            ) else {
                return;
            };
            if !dialog.open() {
                if let Err(error) = open_viewer(&preview, &dialog, &body) {
                    log::warn!("Could not open Mermaid viewer: {error:?}");
                }
            }
        })
    };

    let onclose = {
        let body_ref = body_ref.clone();
        Callback::from(move |_: Event| {
            if let Some(body) = body_ref.cast::<Element>() {
                body.set_inner_html("");
            }
        })
    };

    let close = {
        let dialog_ref = dialog_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(dialog) = dialog_ref.cast::<HtmlDialogElement>() {
                dialog.close();
            }
        })
    };

    let viewer = html! {
        <dialog
            ref={dialog_ref}
            class="dialog dm-mermaid-viewer"
            aria-label="Mermaid diagram at original size (1:1)"
            style="width:calc(100vw - 2rem);max-width:calc(100vw - 2rem);max-height:calc(100dvh - 2rem);box-sizing:border-box;padding:1rem;overflow:hidden;"
            {onclose}
        >
            <div style="display:flex;flex-direction:column;max-height:calc(100dvh - 4rem);gap:1rem;">
                <div class="dm-mermaid-viewer-header" style="display:flex;align-items:center;justify-content:space-between;gap:1rem;flex-shrink:0;">
                    <strong>{ "Original size (1:1)" }</strong>
                    <button type="button" class="btn btn-outline btn-sm" autofocus={true} onclick={close}>{ "Close" }</button>
                </div>
                <div ref={body_ref} class="dm-mermaid-viewer-body" style="overflow:auto;min-height:0;" tabindex="0" aria-label="Scrollable Mermaid diagram" />
            </div>
        </dialog>
    };

    (markdown_ref, onclick, viewer)
}

fn add_zoom_buttons(markdown: &Element) -> Result<(), JsValue> {
    let previews = markdown.query_selector_all(".dm-mermaid-preview")?;
    let document = markdown
        .owner_document()
        .expect("mounted Markdown has a document");
    for index in 0..previews.length() {
        let preview = previews
            .item(index)
            .expect("preview exists")
            .unchecked_into::<Element>();
        let Some(chart) = preview.query_selector(":scope > .dm-mermaid-chart")? else {
            continue;
        };
        if chart.query_selector(":scope > svg")?.is_none()
            || preview
                .query_selector(":scope > .dm-mermaid-zoom-trigger")?
                .is_some()
        {
            continue;
        }
        let button = document.create_element("button")?;
        button.set_attribute("type", "button")?;
        button.set_attribute("class", "btn btn-outline btn-sm dm-mermaid-zoom-trigger")?;
        button.set_attribute("aria-label", "View Mermaid diagram at original size (1:1)")?;
        button.set_attribute("aria-haspopup", "dialog")?;
        button.set_attribute("title", "View at original size (1:1)")?;
        button.set_text_content(Some("1:1"));
        preview.insert_before(&button, Some(&chart))?;
        chart.set_attribute("style", "cursor:zoom-in;")?;
    }
    Ok(())
}

fn open_viewer(
    preview: &Element,
    dialog: &HtmlDialogElement,
    body: &Element,
) -> Result<(), JsValue> {
    let Some(chart) = preview.query_selector(":scope > .dm-mermaid-chart")? else {
        return Ok(());
    };
    let Some(svg) = chart.query_selector(":scope > svg")? else {
        return Ok(());
    };
    let width = svg
        .get_attribute("width")
        .expect("rendered SVG has an intrinsic width");
    let height = svg
        .get_attribute("height")
        .expect("rendered SVG has an intrinsic height");
    let clone = chart
        .clone_node_with_deep(true)?
        .unchecked_into::<Element>();
    clone.set_attribute(
        "style",
        "margin:0;width:max-content;max-width:none;overflow:visible;box-sizing:content-box;",
    )?;
    clone.query_selector("svg")?.expect("cloned chart has an SVG").set_attribute(
        "style",
        &format!("display:block;width:{width}px;height:{height}px;max-width:none;max-height:none;min-width:0;min-height:0;"),
    )?;

    let viewer_id = NEXT_VIEWER_ID.fetch_add(1, Ordering::Relaxed);
    let markers = clone.query_selector_all("marker[id]")?;
    let edges = clone.query_selector_all("[marker-end]")?;
    for index in 0..markers.length() {
        let marker = markers
            .item(index)
            .expect("marker exists")
            .unchecked_into::<Element>();
        let old_id = marker.id();
        let new_id = format!("{old_id}-viewer-{viewer_id}");
        marker.set_id(&new_id);
        for edge_index in 0..edges.length() {
            let edge = edges
                .item(edge_index)
                .expect("edge exists")
                .unchecked_into::<Element>();
            if edge.get_attribute("marker-end").as_deref() == Some(&format!("url(#{old_id})")) {
                edge.set_attribute("marker-end", &format!("url(#{new_id})"))?;
            }
        }
    }

    body.set_inner_html("");
    body.append_child(&clone)?;
    body.set_scroll_left(0);
    body.set_scroll_top(0);
    // Focusing the native trigger lets dialog close/Escape restore focus to it,
    // including when the mouse activated an SVG node rather than the button.
    if let Some(button) = preview.query_selector(":scope > .dm-mermaid-zoom-trigger")? {
        let options = FocusOptions::new();
        options.set_prevent_scroll(true);
        button
            .unchecked_into::<HtmlElement>()
            .focus_with_options(&options)?;
    }
    dialog.show_modal()
}

#[cfg(all(test, target_arch = "wasm32"))]
mod browser_tests {
    use std::time::Duration;

    use wasm_bindgen_test::*;

    use super::*;
    use crate::components::dm_markdown::{DmMarkdown, DmMarkdownProps, FrontMatterMode};

    wasm_bindgen_test_configure!(run_in_browser);

    fn props(markdown: &str) -> DmMarkdownProps {
        DmMarkdownProps {
            class: Classes::default(),
            allow_html: true,
            base_url: None,
            custom_elements: Vec::new(),
            color_chips: true,
            front_matter: FrontMatterMode::Render,
            markdown: markdown.to_owned().into(),
            syntax_service: Default::default(),
            variant: None,
        }
    }

    async fn wait_until(condition: impl Fn() -> bool) {
        for _ in 0..100 {
            if condition() {
                return;
            }
            yew::platform::time::sleep(Duration::from_millis(10)).await;
        }
        assert!(
            condition(),
            "Mermaid viewer did not reach the expected state"
        );
    }

    #[wasm_bindgen_test]
    async fn closes_and_refreshes_viewer_when_markdown_changes() {
        let document = gloo_utils::document();
        let root = document.create_element("div").unwrap();
        document.body().unwrap().append_child(&root).unwrap();
        let mut app = yew::Renderer::<DmMarkdown>::with_root_and_props(
            root.clone(),
            props("```mermaid\nflowchart LR\nA[Before] --> B[Update]\n```"),
        )
        .render();
        wait_until(|| {
            root.query_selector(".dm-mermaid-zoom-trigger")
                .unwrap()
                .is_some()
        })
        .await;
        let trigger = root
            .query_selector(".dm-mermaid-zoom-trigger")
            .unwrap()
            .unwrap();
        assert!(!trigger.parent_element().unwrap().has_attribute("role"));
        let dialog = root
            .query_selector("dialog")
            .unwrap()
            .unwrap()
            .unchecked_into::<HtmlDialogElement>();
        trigger.unchecked_ref::<HtmlElement>().click();
        wait_until(|| dialog.open()).await;
        assert!(dialog.text_content().unwrap().contains("Before"));
        let source_marker = root
            .query_selector(".dm-mermaid-preview marker")
            .unwrap()
            .unwrap()
            .id();
        assert_ne!(
            source_marker,
            dialog.query_selector("marker").unwrap().unwrap().id()
        );

        app.update(props(
            "```mermaid\nflowchart TD\nX[After] --> Y[Update]\n```",
        ));
        wait_until(|| !dialog.open() && root.text_content().unwrap().contains("After")).await;
        assert_eq!(
            root.query_selector_all(".dm-mermaid-zoom-trigger")
                .unwrap()
                .length(),
            1
        );
        assert!(dialog.query_selector("svg").unwrap().is_none());
        root.query_selector(".dm-mermaid-zoom-trigger")
            .unwrap()
            .unwrap()
            .unchecked_ref::<HtmlElement>()
            .click();
        wait_until(|| dialog.open()).await;
        assert!(dialog.text_content().unwrap().contains("After"));
        assert!(!dialog.text_content().unwrap().contains("Before"));

        app.destroy();
        wait_until(|| root.first_element_child().is_none()).await;
        assert!(!dialog.is_connected());
        root.remove();
    }
}
