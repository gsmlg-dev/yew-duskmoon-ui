#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_PlaneCar)]
pub fn r#icon_plane_car(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21.57 12.66C21.43 12.26 21.05 12 20.6 12H13.41C12.95 12 12.58 12.26 12.43 12.66L11 16.77V22.28C11 22.66 11.32 23 11.7 23H12.32C12.7 23 13 22.62 13 22.24V21H21V22.24C21 22.62 21.31 23 21.69 23H22.3C22.68 23 23 22.66 23 22.28V16.77L21.57 12.66M13.41 13H20.6L21.63 16H12.38L13.41 13M13 19C12.45 19 12 18.55 12 18S12.45 17 13 17 14 17.45 14 18 13.55 19 13 19M21 19C20.45 19 20 18.55 20 18S20.45 17 21 17 22 17.45 22 18 21.55 19 21 19M6.66 14.53L7 17L5.95 18.06L4.19 14.88L1 13.11L2.06 12.03L4.56 12.4L8.43 8.53L1 4.62L2.42 3.21L11.61 5.33L15.5 1.44C16.06 .855 17.06 .855 17.62 1.44C18.21 2.03 18.21 3 17.62 3.56L13.73 7.45L14.55 11H13.41C12.54 11 11.79 11.5 11.5 12.31L11.47 12.37L10.56 10.63L6.66 14.53Z" />
    </svg>
  }
}
