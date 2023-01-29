#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_TallyMark4)]
pub fn r#icon_tally_mark_4(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7 19H5V5H7V19M11 5H9V19H11V5M15 5H13V19H15V5M19 5H17V19H19V5Z" />
    </svg>
  }
}
