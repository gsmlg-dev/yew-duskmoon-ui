#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_GoKartTrack)]
pub fn r#icon_go_kart_track(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22,5.5A3.5,3.5 0 0,0 18.5,2A3.5,3.5 0 0,0 15,5.5V6A3,3 0 0,1 12,9C10,9 9,6 6,6A4,4 0 0,0 2,10V11H4V10A2,2 0 0,1 6,8C6.86,8 7.42,8.45 8.32,9.24C9.28,10.27 10.6,10.9 12,11A5,5 0 0,0 17,6V5.5A1.5,1.5 0 0,1 18.5,4A1.5,1.5 0 0,1 20,5.5C19.86,6.35 19.58,7.18 19.17,7.94C18.5,9.2 18.11,10.58 18,12C18.09,13.37 18.5,14.71 19.21,15.89C19.6,16.54 19.87,17.25 20,18A2,2 0 0,1 18,20A2,2 0 0,1 16,18A3.75,3.75 0 0,0 12.25,14.25A3.75,3.75 0 0,0 8.5,18V18.5A1.5,1.5 0 0,1 7,20A3,3 0 0,1 4,17V15H6V13H0V15H2V17A5,5 0 0,0 7,22A3.5,3.5 0 0,0 10.5,18.5V18A1.75,1.75 0 0,1 12.25,16.25A1.75,1.75 0 0,1 14,18A4,4 0 0,0 18,22A4,4 0 0,0 22,18C22,16 20,14 20,12C20,10 22,7.5 22,5.5Z" />
    </svg>
  }
}
