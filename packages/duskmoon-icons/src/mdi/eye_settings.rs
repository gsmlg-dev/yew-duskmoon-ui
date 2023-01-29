#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_EyeSettings)]
pub fn r#icon_eye_settings(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,9A3,3 0 0,0 9,12A3,3 0 0,0 12,15A3,3 0 0,0 15,12A3,3 0 0,0 12,9M12,17A5,5 0 0,1 7,12A5,5 0 0,1 12,7A5,5 0 0,1 17,12A5,5 0 0,1 12,17M12,4.5C7.14,4.5 2.78,7.5 1,12C3.39,18.08 10.25,21.06 16.33,18.67C19.38,17.47 21.8,15.06 23,12C21.22,7.5 16.86,4.5 12,4.5M7,22H9V24H7V22M11,22H13V24H11V22M15,22H17V24H15V22Z" />
    </svg>
  }
}