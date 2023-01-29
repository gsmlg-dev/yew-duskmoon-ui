#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_BriefcaseOff)]
pub fn r#icon_briefcase_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.11 21.46L20.84 22.73L19.11 21H4C2.89 21 2 20.1 2 19V8C2 6.89 2.89 6 4 6H4.11L1.11 3L2.39 1.73L22.11 21.46M22 18.8L8 4.8V4C8 2.89 8.89 2 10 2H14C15.11 2 16 2.9 16 4V6H20C21.11 6 22 6.9 22 8V18.8M14 4H10V6H14V4Z" />
    </svg>
  }
}
