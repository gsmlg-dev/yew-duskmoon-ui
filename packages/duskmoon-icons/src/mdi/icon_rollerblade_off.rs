#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_RollerbladeOff)]
pub fn r#icon_rollerblade_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20.84 22.73L20.32 22.21C19.86 22.69 19.22 23 18.5 23C17.12 23 16 21.88 16 20.5C16 19.78 16.31 19.14 16.79 18.68L15.11 17H3V4.9L1.11 3L2.39 1.73L22.11 21.46L20.84 22.73M19.5 12.95C18.55 10.9 15.82 10.56 13.91 10.05C13.45 10.03 13 9.77 12.6 9.4L20 16.79C20 15.45 20.06 13.76 19.5 12.95M9.2 6L10.2 7H11.5V6H9.2M12.56 9.36C12.18 9 11.9 8.5 11.82 8H11.2L12.56 9.36M11.5 18C10.12 18 9 19.12 9 20.5C9 21.88 10.12 23 11.5 23S14 21.88 14 20.5C14 19.12 12.88 18 11.5 18M9 5H11.5V2H5.2L8.5 5.32C8.59 5.13 8.77 5 9 5M4.5 18C3.12 18 2 19.12 2 20.5C2 21.88 3.12 23 4.5 23S7 21.88 7 20.5C7 19.12 5.88 18 4.5 18Z" />
    </svg>
  }
}
