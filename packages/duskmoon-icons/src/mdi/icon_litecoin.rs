#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Litecoin)]
pub fn r#icon_litecoin(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12.55,15.92L13.16,13.78L14.81,13.19L16.25,7.63L14.56,8.31L16.25,2H8L5.38,11.77L3.72,12.34L2.22,17.91L4,17.27L2.66,22H20.16L21.78,15.92H12.55M19.39,21H4L5.46,15.65L3.72,16.3L4.58,13.11L6.24,12.54L8.74,3H15L13.13,10L14.83,9.3L14,12.42L12.33,13L11.22,16.91H20.5L19.39,21Z" />
    </svg>
  }
}
