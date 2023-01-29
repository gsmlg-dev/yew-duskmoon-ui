#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Firework)]
pub fn r#icon_firework(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5.8,16.59L4.5,15.28L12.26,7.5L16.5,11.74L8.72,19.5L7.29,18.09C7.04,18.16 6.8,18.28 6.63,18.5C6.57,18.57 6.5,18.65 6.5,18.74C6.42,18.88 6.38,19 6.32,19.15C6.21,19.42 6.09,19.69 5.93,19.93C5.81,20.1 5.68,20.26 5.53,20.39C5.42,20.5 5.29,20.59 5.16,20.66C5.08,20.71 5,20.76 4.9,20.79C4.3,21.04 3.63,21 3,21V19C3.23,19 3.83,19 3.9,19C4,19 4.08,19 4.16,18.94C4.18,18.92 4.19,18.91 4.21,18.89C4.28,18.81 4.34,18.7 4.39,18.6C4.47,18.42 4.53,18.24 4.6,18.06L4.64,17.96C4.76,17.69 4.9,17.45 5.08,17.23C5.18,17.1 5.3,17 5.42,16.87C5.54,16.77 5.66,16.67 5.8,16.59M21,3L19.88,11.19L12.81,4.12L21,3Z" />
    </svg>
  }
}
