#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Reproduction)]
pub fn r#icon_reproduction(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12.72,13.15L13.62,12.26C13.6,11 14.31,9.44 15.62,8.14C17.57,6.18 20.11,5.55 21.28,6.72C22.45,7.89 21.82,10.43 19.86,12.38C18.56,13.69 17,14.4 15.74,14.38L14.85,15.28C14.5,15.61 14,15.66 13.6,15.41C12.76,15.71 12,16.08 11.56,16.8C11.03,17.68 11.03,19.1 10.47,19.95C9.91,20.81 8.79,21.1 7.61,21.1C6.43,21.1 5,21 3.95,19.5L6.43,19.92C7,20 8.5,19.39 9.05,18.54C9.61,17.68 9.61,16.27 10.14,15.38C10.61,14.6 11.5,14.23 12.43,13.91C12.42,13.64 12.5,13.36 12.72,13.15M7,2A5,5 0 0,1 12,7A5,5 0 0,1 7,12A5,5 0 0,1 2,7A5,5 0 0,1 7,2M7,4A3,3 0 0,0 4,7A3,3 0 0,0 7,10A3,3 0 0,0 10,7A3,3 0 0,0 7,4Z" />
    </svg>
  }
}
