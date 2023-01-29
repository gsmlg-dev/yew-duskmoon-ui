#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ZodiacAries)]
pub fn r#icon_zodiac_aries(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16,2C14.95,2 13.91,2.27 13,2.81C12.64,3 12.3,3.26 12,3.54C11.7,3.26 11.36,3 11,2.81C10.09,2.27 9.05,2 8,2A6,6 0 0,0 2,8A6,6 0 0,0 8,14V12A4,4 0 0,1 4,8A4,4 0 0,1 8,4C9,4 10,4.39 10.75,5.1C10.84,5.18 10.92,5.27 11,5.36V22H13V5.36C13.08,5.27 13.16,5.18 13.25,5.1C14.85,3.58 17.38,3.64 18.91,5.25C20.43,6.85 20.36,9.38 18.76,10.9C18,11.61 17.03,12 16,12V14A6,6 0 0,0 22,8A6,6 0 0,0 16,2Z" />
    </svg>
  }
}
