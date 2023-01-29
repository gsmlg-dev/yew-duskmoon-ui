#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_PaperclipOff)]
pub fn r#icon_paperclip_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8.5 5.3L7.16 3.96C7.62 2.26 9.15 1 11 1C13.21 1 15 2.79 15 5V11.8L13.5 10.3V5C13.5 3.62 12.38 2.5 11 2.5S8.5 3.62 8.5 5V5.3M18 6H16.5V13.3L18 14.8V6M22.11 21.46L20.84 22.73L17.62 19.5C16.81 21.55 14.83 23 12.5 23C9.46 23 7 20.54 7 17.5V8.89L1.11 3L2.39 1.73L22.11 21.46M11.5 15.5C11.5 16.05 11.95 16.5 12.5 16.5S13.5 16.05 13.5 15.5V15.39L11.5 13.39V15.5M16.42 18.31L14.73 16.62C14.32 17.43 13.5 18 12.5 18C11.12 18 10 16.88 10 15.5V11.89L8.5 10.39V17.5C8.5 19.71 10.29 21.5 12.5 21.5C14.43 21.5 16.04 20.13 16.42 18.31M10 6.8L11.5 8.3V6H10V6.8Z" />
    </svg>
  }
}
