#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_ShieldSearch)]
pub fn r#icon_shield_search(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,9A3,3 0 0,1 15,12A3,3 0 0,1 12,15A3,3 0 0,1 9,12A3,3 0 0,1 12,9M17.86,19.31C16.23,21.22 14.28,22.45 12,23C9.44,22.39 7.3,20.93 5.58,18.63C3.86,16.34 3,13.8 3,11V5L12,1L21,5V11C21,13.39 20.36,15.61 19.08,17.67L16.17,14.76C16.69,13.97 17,13 17,12A5,5 0 0,0 12,7A5,5 0 0,0 7,12A5,5 0 0,0 12,17C13,17 13.97,16.69 14.76,16.17L17.86,19.31Z" />
    </svg>
  }
}
