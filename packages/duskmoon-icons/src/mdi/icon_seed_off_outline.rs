#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_SeedOffOutline)]
pub fn r#icon_seed_off_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.1 21.5L2.4 1.7L1.1 3L5.9 7.8C3.8 10.5 2.6 14.6 3.2 20.8C4.3 20.9 5.4 21 6.4 21C10.9 21 14.1 19.9 16.3 18.2L20.9 22.8L22.1 21.5M6.4 19H5.1C4.9 14.8 5.6 11.5 7.3 9.2L8.9 10.8C7 13.7 7 17 7 17C8 15 9.1 13.4 10.2 12.1L14.8 16.7C12.8 18.2 10 19 6.4 19M10 6.8L8.5 5.3C11.3 3.4 14.7 3 17.2 3C19.3 3 20.7 3.3 20.7 3.3S22.1 10.3 18.7 15.5L17.2 14C19.1 10.9 19 7.1 18.9 5.1C18.4 5 17.8 5 17.2 5C15.1 5 12.3 5.3 10 6.8M12.8 9.6L11.6 8.4C12.9 7.5 14.7 7 17 7C17 7 15.1 7.6 12.8 9.6Z" />
    </svg>
  }
}
