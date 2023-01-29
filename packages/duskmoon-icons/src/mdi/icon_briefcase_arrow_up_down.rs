#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_BriefcaseArrowUpDown)]
pub fn r#icon_briefcase_arrow_up_down(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10 2H14C15.11 2 16 2.9 16 4V6H20C21.11 6 22 6.9 22 8V13.53C20.94 12.58 19.54 12 18 12C14.69 12 12 14.69 12 18C12 19.09 12.29 20.12 12.8 21H4C2.89 21 2 20.1 2 19V8C2 6.89 2.89 6 4 6H8V4C8 2.89 8.89 2 10 2M14 6V4H10V6H14M17.5 19H19V15H21V19H22.5L20 22L17.5 19M17 17V21H15V17H13.5L16 14L18.5 17H17Z" />
    </svg>
  }
}
