#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FlaskRoundBottom)]
pub fn r#icon_flask_round_bottom(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 15C19 18.87 15.87 22 12 22C8.13 22 5 18.87 5 15C5 12.21 6.64 9.8 9 8.67V5C9 4.45 9.45 4 10 4H10.5L9.5 2H14.5L13.5 4H14C14.55 4 15 4.45 15 5V8.67C17.36 9.8 19 12.21 19 15M11 6V10.1C8.72 10.56 7 12.58 7 15L7.08 15.91L9 13.93L13.07 18L16.93 14.14C16.58 12.12 15 10.5 13 10.1V6H11M13.07 12C13.62 12 14.07 12.45 14.07 13C14.07 13.55 13.62 14 13.07 14C12.5 14 12.07 13.55 12.07 13C12.07 12.45 12.5 12 13.07 12Z" />
    </svg>
  }
}
