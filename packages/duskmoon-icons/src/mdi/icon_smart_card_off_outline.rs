#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SmartCardOffOutline)]
pub fn r#icon_smart_card_off_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 13H16.2L15 11.8V10H19V13M22 3H6.2L8.2 5H22V18.8L23.5 20.29C23.79 19.94 24 19.5 24 19V5C23.96 3.91 23.09 3.04 22 3M22.11 21.46L20.84 22.73L19.11 21H2C.911 20.96 .036 20.09 0 19V5C.028 4.17 .545 3.47 1.27 3.16L1.11 3L2.39 1.73L22.11 21.46M17.11 19L14 15.89V17H4V15.75C4 14.09 7.34 13.25 9 13.25C9.78 13.25 10.91 13.44 11.91 13.8L9.91 11.8C9.63 11.92 9.33 12 9 12C7.62 12 6.5 10.88 6.5 9.5C6.5 9.17 6.58 8.87 6.7 8.59L3.11 5H2V19H17.11Z" />
    </svg>
  }
}
