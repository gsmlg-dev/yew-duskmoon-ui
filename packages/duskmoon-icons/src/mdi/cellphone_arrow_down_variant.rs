#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_CellphoneArrowDownVariant)]
pub fn r#icon_cellphone_arrow_down_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18 14L23 9L21.6 7.6L19 10.2V3H17V10.2L14.4 7.6L13 9L18 14M19 16V21C19 22.1 18.1 23 17 23H7C5.9 23 5 22.1 5 21V3C5 1.9 5.9 1 7 1H14V5H7V19H17V16H19Z" />
    </svg>
  }
}
