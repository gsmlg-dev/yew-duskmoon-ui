#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_RestoreAlert)]
pub fn r#icon_restore_alert(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M13 3C8 3 4 7 4 12H1L4.9 15.9L5 16L9 12H6C6 8.1 9.1 5 13 5S20 8.1 20 12 16.9 19 13 19C11.1 19 9.3 18.2 8.1 16.9L6.7 18.3C8.3 20 10.5 21 13 21C18 21 22 17 22 12S18 3 13 3M12 15H14V17H12V15M12 7H14V13H12V7" />
    </svg>
  }
}
