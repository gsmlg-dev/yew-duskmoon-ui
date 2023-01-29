#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TimerCancel)]
pub fn r#icon_timer_cancel(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 3H9V1H15V3M12 18.5C12 19.77 12.37 20.94 13 21.94C12.67 22 12.34 22 12 22C7.03 22 3 17.97 3 13S7.03 4 12 4C14.12 4 16.07 4.74 17.62 6L19.04 4.56C19.55 5 20 5.46 20.45 5.97L19.03 7.39C20.16 8.81 20.87 10.57 21 12.5C20.22 12.18 19.38 12 18.5 12C14.91 12 12 14.91 12 18.5M13 7H11V14H13V7M23 18.5C23 21 21 23 18.5 23S14 21 14 18.5 16 14 18.5 14 23 16 23 18.5M20 21.08L15.92 17C15.65 17.42 15.5 17.94 15.5 18.5C15.5 20.16 16.84 21.5 18.5 21.5C19.06 21.5 19.58 21.35 20 21.08M21.5 18.5C21.5 16.84 20.16 15.5 18.5 15.5C17.94 15.5 17.42 15.65 17 15.92L21.08 20C21.35 19.58 21.5 19.06 21.5 18.5Z" />
    </svg>
  }
}
