use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FileInputProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub variant: Option<String>,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub aria_label: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub accept: AttrValue,
    #[prop_or_default]
    pub multiple: bool,
    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(FileInput)]
pub fn file_input(props: &FileInputProps) -> Html {
    let mut classes = classes!("file-input");
    if let Some(variant) = &props.variant {
        classes.push(format!("file-input-{}", variant));
    }
    classes.push(props.class.clone());
    html! {
        <input class={classes} type="file" id={props.id.clone()} name={props.name.clone()}
            aria-label={props.aria_label.clone()} accept={props.accept.clone()} multiple={props.multiple}
            disabled={props.disabled} required={props.required} onchange={props.onchange.clone()} />
    }
}
