#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Signal2g)]
pub fn r#icon_signal_2g(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M11,19.5H2V13.5A3,3 0 0,1 5,10.5H8V7.5H2V4.5H8A3,3 0 0,1 11,7.5V10.5A3,3 0 0,1 8,13.5H5V16.5H11M22,10.5H17.5V13.5H19V16.5H16V7.5H22V4.5H16A3,3 0 0,0 13,7.5V16.5A3,3 0 0,0 16,19.5H19A3,3 0 0,0 22,16.5" />
    </svg>
  }
}
