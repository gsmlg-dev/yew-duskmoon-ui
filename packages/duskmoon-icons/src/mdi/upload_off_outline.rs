#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_UploadOffOutline)]
pub fn r#icon_upload_off_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.11 21.46L2.39 1.73L1.11 3L6.56 8.45L5 10H8.11L9 10.89V16H14.11L16.11 18H5V20H18.11L20.84 22.73L22.11 21.46M11 14V12.89L12.11 14H11M12 5.8L14.2 8H13V9.8L15 11.8V10H19L12 3L9.1 5.9L10.5 7.3L12 5.8Z" />
    </svg>
  }
}
