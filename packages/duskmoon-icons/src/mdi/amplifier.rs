#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Amplifier)]
pub fn r#icon_amplifier(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10,2H14A1,1 0 0,1 15,3H21V21H19A1,1 0 0,1 18,22A1,1 0 0,1 17,21H7A1,1 0 0,1 6,22A1,1 0 0,1 5,21H3V3H9A1,1 0 0,1 10,2M5,5V9H19V5H5M7,6A1,1 0 0,1 8,7A1,1 0 0,1 7,8A1,1 0 0,1 6,7A1,1 0 0,1 7,6M12,6H14V7H12V6M15,6H16V8H15V6M17,6H18V8H17V6M12,11A4,4 0 0,0 8,15A4,4 0 0,0 12,19A4,4 0 0,0 16,15A4,4 0 0,0 12,11M10,6A1,1 0 0,1 11,7A1,1 0 0,1 10,8A1,1 0 0,1 9,7A1,1 0 0,1 10,6Z" />
    </svg>
  }
}