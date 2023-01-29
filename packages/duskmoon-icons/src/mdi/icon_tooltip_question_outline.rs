#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_TooltipQuestionOutline)]
pub fn r#icon_tooltip_question_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M20 2H4C3.47 2 2.96 2.21 2.59 2.59C2.21 2.96 2 3.47 2 4V16C2 16.53 2.21 17.04 2.59 17.41C2.96 17.79 3.47 18 4 18H8L12 22L16 18H20C20.53 18 21.04 17.79 21.41 17.41S22 16.53 22 16V4C22 3.47 21.79 2.96 21.41 2.59C21.04 2.21 20.53 2 20 2M4 16V4H20V16H15.17L12 19.17L8.83 16M10.05 6.04C10.59 5.68 11.3 5.5 12.19 5.5C13.13 5.5 13.88 5.71 14.42 6.12C14.96 6.54 15.23 7.1 15.23 7.8C15.23 8.24 15.08 8.63 14.79 9C14.5 9.36 14.12 9.64 13.66 9.85C13.4 10 13.23 10.15 13.14 10.32C13.05 10.5 13 10.72 13 11H11C11 10.5 11.1 10.16 11.29 9.92C11.5 9.68 11.84 9.4 12.36 9.08C12.62 8.94 12.83 8.76 13 8.54C13.14 8.33 13.22 8.08 13.22 7.8C13.22 7.5 13.13 7.28 12.95 7.11C12.77 6.93 12.5 6.85 12.19 6.85C11.92 6.85 11.7 6.92 11.5 7.06C11.34 7.2 11.24 7.41 11.24 7.69H9.27C9.22 7 9.5 6.4 10.05 6.04M11 14V12H13V14Z" />
    </svg>
  }
}
