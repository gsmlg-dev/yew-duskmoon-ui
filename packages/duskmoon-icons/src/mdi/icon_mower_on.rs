#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MowerOn)]
pub fn r#icon_mower_on(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 2H14V7H12V2M22 6.3L20.6 4.9L17 8.4L18.4 9.8C18.4 9.8 21.9 6.3 22 6.3M18.5 14C17.5 14 16.7 14.4 16.1 15H9.4L5.7 3H2V5H4.3L7 14H6.5C4.6 14 3 15.6 3 17.5S4.6 21 6.5 21C7.9 21 9.1 20.2 9.7 19H15.4C16 20.2 17.2 21 18.6 21C20.5 21 22.1 19.4 22.1 17.5S20.4 14 18.5 14M6.5 19C5.7 19 5 18.3 5 17.5S5.7 16 6.5 16 8 16.7 8 17.5 7.3 19 6.5 19M18.5 19C17.7 19 17 18.3 17 17.5S17.7 16 18.5 16 20 16.7 20 17.5 19.3 19 18.5 19M9.5 12L10.1 14H15L14.6 13.1C14.3 12.4 13.5 12 12.8 12H9.5Z" />
    </svg>
  }
}
