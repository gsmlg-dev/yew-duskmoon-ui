#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_AlphabetTengwar)]
pub fn r#icon_alphabet_tengwar(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10 7L12.5 3H15L11 7H10M12.5 9C11.96 9 11.46 9.13 11 9.35V9H8V11H9V21H11V12.5C11 11.67 11.67 11 12.5 11C13.33 11 14 11.67 14 12.5V14.5C14 15.33 13.33 16 12.5 16H12V18H12.5C14.43 18 16 16.43 16 14.5V12.5C16 10.57 14.43 9 12.5 9Z" />
    </svg>
  }
}
