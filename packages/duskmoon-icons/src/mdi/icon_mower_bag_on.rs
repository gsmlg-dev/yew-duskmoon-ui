#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_MowerBagOn)]
pub fn r#icon_mower_bag_on(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 2H14V7H12V2M22 6.3L20.6 4.9L17 8.4L18.4 9.8C18.4 9.8 21.9 6.3 22 6.3M10.8 12L11.9 14H17L16.5 13.1C16.2 12.4 15.5 12 14.7 12H10.8M20 15H11.4L4.6 3H1V5H3.4L7.4 12.1L2.3 10.1C2.1 10 1.7 10 1.4 10.2C1.2 10.4 1 10.7 1 11V18C1 18.5 1.5 19 2 19H5.3C5.9 20.2 7.1 21 8.5 21S11.1 20.2 11.7 19H17.2C17.6 20.2 18.7 21 20 21C21.7 21 23 19.7 23 18S21.7 15 20 15M5 17H3V12.5L7.3 14.2C6.1 14.6 5.2 15.7 5 17M8.5 19C7.7 19 7 18.3 7 17.5S7.7 16 8.5 16 10 16.7 10 17.5 9.3 19 8.5 19M20 19C19.5 19 19 18.5 19 18S19.5 17 20 17 21 17.5 21 18 20.5 19 20 19Z" />
    </svg>
  }
}
