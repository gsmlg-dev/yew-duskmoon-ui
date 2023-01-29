#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CandyOutline)]
pub fn r#icon_candy_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M9.88 9.88C11.05 8.71 12.95 8.71 14.12 9.88S15.29 12.95 14.12 14.12 11.05 15.29 9.88 14.12 8.71 11.05 9.88 9.88M8.46 8.46C6.5 10.42 6.5 13.58 8.46 15.54S13.58 17.5 15.54 15.54 17.5 10.42 15.54 8.46 10.42 6.5 8.46 8.46M19.47 4.55C19.47 4.55 18.5 4.67 17.43 5.37C17.28 4.32 16.78 3.27 15.93 2.42C14.68 3.67 14.53 5.22 14.83 6.34C16.22 6.7 17.3 7.78 17.66 9.17C18.78 9.47 20.34 9.32 21.58 8.07C20.74 7.23 19.71 6.74 18.68 6.58C19.07 6 19.38 5.33 19.47 4.55M4.53 19.45C4.53 19.45 5.5 19.33 6.57 18.64C6.72 19.68 7.22 20.73 8.07 21.58C9.32 20.34 9.47 18.78 9.17 17.66C7.79 17.3 6.7 16.22 6.34 14.83C5.22 14.53 3.67 14.68 2.42 15.93C3.26 16.77 4.29 17.27 5.32 17.42C4.93 18 4.62 18.68 4.53 19.45Z" />
    </svg>
  }
}
