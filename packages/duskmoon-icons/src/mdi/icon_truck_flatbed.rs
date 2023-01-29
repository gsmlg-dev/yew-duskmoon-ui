#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_TruckFlatbed)]
pub fn r#icon_truck_flatbed(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 4H13V13H1V17H3C3 17.83 3.3 18.53 3.89 19.13C4.5 19.72 5.19 20 6 20S7.5 19.72 8.11 19.13C8.7 18.53 9 17.83 9 17H14.5C14.5 17.83 14.78 18.53 15.38 19.13C15.97 19.72 16.67 20 17.5 20C18.3 20 19 19.72 19.59 19.13C20.19 18.53 20.5 17.83 20.5 17H23V10L18 4M7.08 18.07C6.8 18.37 6.44 18.5 6 18.5S5.2 18.37 4.92 18.07C4.64 17.77 4.5 17.42 4.5 17C4.5 16.61 4.64 16.26 4.92 15.96C5.2 15.66 5.56 15.5 6 15.5S6.8 15.66 7.08 15.96C7.36 16.26 7.5 16.61 7.5 17C7.5 17.42 7.36 17.77 7.08 18.07M18.54 18.07C18.24 18.37 17.89 18.5 17.5 18.5C17.08 18.5 16.73 18.37 16.43 18.07S16 17.42 16 17C16 16.61 16.13 16.26 16.43 15.96C16.73 15.66 17.08 15.5 17.5 15.5C17.89 15.5 18.24 15.66 18.54 15.96C18.84 16.26 19 16.61 19 17C19 17.42 18.84 17.77 18.54 18.07M15 10V6H17.06L20.39 10H15Z" />
    </svg>
  }
}
