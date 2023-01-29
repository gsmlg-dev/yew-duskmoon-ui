#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CarSeat)]
pub fn r#icon_car_seat(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7 18C7 18 4 10 4 6S6 2 6 2H7C7 2 8 2 8 3S7 4 7 6 10 10 10 13 7 18 7 18M12 17C11 17 8 19.5 8 19.5C7.7 19.7 7.8 20 8 20.3C8 20.3 9 22.1 11 22.1H17C18.1 22.1 19 21.2 19 20.1V19.1C19 18 18.1 17.1 17 17.1H12Z" />
    </svg>
  }
}
