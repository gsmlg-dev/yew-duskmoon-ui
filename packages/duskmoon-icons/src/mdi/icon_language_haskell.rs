#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_LanguageHaskell)]
pub fn r#icon_language_haskell(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2.08,19L6.75,12L2.08,5H5.58L10.25,12L5.58,19H2.08M6.75,19L11.42,12L6.75,5H10.25L19.59,19H16.09L13.17,14.63L10.25,19H6.75M18.03,14.92L16.5,12.58H21.92V14.92H18.03M15.7,11.42L14.14,9.08H21.92V11.42H15.7Z" />
    </svg>
  }
}
