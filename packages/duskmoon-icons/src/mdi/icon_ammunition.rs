#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Ammunition)]
pub fn r#icon_ammunition(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M14,22H10V21H14V22M13,10V7H11V10L10,11.5V20H14V11.5L13,10M12,2C12,2 11,3 11,5V6H13V5C13,5 13,3 12,2M8,22H4V21H8V22M7,10V7H5V10L4,11.5V20H8V11.5L7,10M6,2C6,2 5,3 5,5V6H7V5C7,5 7,3 6,2M20,22H16V21H20V22M19,10V7H17V10L16,11.5V20H20V11.5L19,10M18,2C18,2 17,3 17,5V6H19V5C19,5 19,3 18,2Z" />
    </svg>
  }
}
