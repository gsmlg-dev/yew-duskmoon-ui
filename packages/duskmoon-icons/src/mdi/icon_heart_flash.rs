#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_HeartFlash)]
pub fn r#icon_heart_flash(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16.5,2.83C14.76,2.83 13.09,3.64 12,4.9C10.91,3.64 9.24,2.83 7.5,2.83C4.42,2.83 2,5.24 2,8.33C2,12.1 5.4,15.19 10.55,19.86L12,21.17L13.45,19.86C18.6,15.19 22,12.1 22,8.33C22,5.24 19.58,2.83 16.5,2.83M12,17.83V13.83H9L12,6.83V10.83H15" />
    </svg>
  }
}
