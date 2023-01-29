#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CarParkingLights)]
pub fn r#icon_car_parking_lights(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7.3,9.2C8.1,9.6 9,10.5 9,12C9,13.5 8.1,14.4 7.3,14.8C6.9,13.4 6.9,10.6 7.3,9.2M6.5,7C4.5,7 4.5,17 6.5,17C8.5,17 11,15.1 11,12C11,8.9 8.5,7 6.5,7M16.7,9.2C17,10.6 17,13.4 16.7,14.8C15.9,14.4 15,13.5 15,12C15,10.5 15.9,9.6 16.7,9.2M17.5,7C15.5,7 13,8.9 13,12C13,15.1 15.5,17 17.5,17C19.5,17 19.5,7 17.5,7M4.9,6.2L2.5,4.6L1.4,6.3L4,8C4.2,7.3 4.5,6.6 4.9,6.2M20,8L22.6,6.3L21.5,4.6L19.1,6.2C19.4,6.6 19.8,7.2 20,8M4,16L1.4,17.7L2.5,19.4L4.9,17.8C4.6,17.4 4.2,16.8 4,16M20.5,11C20.5,11.3 20.5,11.7 20.5,12C20.5,12.3 20.5,12.6 20.5,13H24V11H20.5M19.1,17.8L21.5,19.4L22.6,17.7L20,16C19.8,16.7 19.5,17.4 19.1,17.8M3.5,12C3.5,11.7 3.5,11.4 3.5,11H0V13H3.5C3.5,12.7 3.5,12.3 3.5,12Z" />
    </svg>
  }
}
