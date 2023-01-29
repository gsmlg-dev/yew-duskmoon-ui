#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_AttachmentLock)]
pub fn r#icon_attachment_lock(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 13C16.6 13 15.2 14.1 15.2 15.5V17C14.6 17 14 17.6 14 18.2V21.7C14 22.4 14.6 23 15.2 23H20.7C21.4 23 22 22.4 22 21.8V18.3C22 17.6 21.4 17 20.8 17V15.5C20.8 14.1 19.4 13 18 13M18 14.2C18.8 14.2 19.5 14.7 19.5 15.5V17H16.5V15.5C16.5 14.7 17.2 14.2 18 14.2M7.5 5C4.5 5 2 7.5 2 10.5S4.5 16 7.5 16H13.2V15.5C13.2 15.2 13.2 14.8 13.3 14.5H7.5C5.3 14.5 3.5 12.7 3.5 10.5S5.3 6.5 7.5 6.5H18C19.4 6.5 20.5 7.6 20.5 9C20.5 9.9 20 10.7 19.2 11.2C19.8 11.3 20.3 11.6 20.8 11.9C21.6 11.1 22 10.1 22 9C22 6.8 20.2 5 18 5H7.5M9.5 8C8.1 8 7 9.1 7 10.5S8.1 13 9.5 13H14C14.5 12.3 15.1 11.8 15.8 11.5H9.5C8.9 11.5 8.5 11.1 8.5 10.5S8.9 9.5 9.5 9.5H17V8H9.5Z" />
    </svg>
  }
}