#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Submarine)]
pub fn r#icon_submarine(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M8 3.67C6.78 4.53 5.39 4.93 4 5H2V7H4C5.37 7 6.74 6.65 8 6C10.5 7.3 13.5 7.3 16 6C17.26 6.65 18.62 6.94 20 7H22V5H20C18.61 5 17.22 4.53 16 3.67C13.56 5.38 10.44 5.38 8 3.67M16 8.67L15 10V12H14L12 14H9L6 16L5 14H4V16L2 17L4 18V20H5.14L6 18L9 20H20A2 2 0 0 0 22 18V16A2 2 0 0 0 20 14H18L17 13V12H16V10H17V8.67Z" />
    </svg>
  }
}
