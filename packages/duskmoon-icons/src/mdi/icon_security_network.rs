#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_SecurityNetwork)]
pub fn r#icon_security_network(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13,19H14A1,1 0 0,1 15,20H22V22H15A1,1 0 0,1 14,23H10A1,1 0 0,1 9,22H2V20H9A1,1 0 0,1 10,19H11V17.34C8.07,16.13 6,13 6,9.67V5.67L12,3L18,5.67V9.67C18,13 15.93,16.13 13,17.34V19M12,5L8,6.69V10H12V5M12,10V16C13.91,15.53 16,13.06 16,11V10H12Z" />
    </svg>
  }
}
