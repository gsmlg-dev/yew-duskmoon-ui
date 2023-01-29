#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_TextSearchVariant)]
pub fn r#icon_text_search_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9 6V8H2V6H9M9 11V13H2V11H9M18 16V18H2V16H18M19.31 11.5C19.75 10.82 20 10 20 9.11C20 6.61 18 4.61 15.5 4.61S11 6.61 11 9.11 13 13.61 15.5 13.61C16.37 13.61 17.19 13.36 17.88 12.93L21 16L22.39 14.61L19.31 11.5M15.5 11.61C14.12 11.61 13 10.5 13 9.11S14.12 6.61 15.5 6.61 18 7.73 18 9.11 16.88 11.61 15.5 11.61Z" />
    </svg>
  }
}
