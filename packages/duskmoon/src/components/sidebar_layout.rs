use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SidebarLayoutProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub variant: Option<String>,
}

#[function_component(SidebarLayout)]
pub fn sidebar_layout(props: &SidebarLayoutProps) -> Html {
    let mut classes = classes!("sidebar-layout");
    if let Some(variant) = &props.variant {
        classes.push(format!("sidebar-layout-{}", variant));
    }
    classes.push(props.class.clone());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
