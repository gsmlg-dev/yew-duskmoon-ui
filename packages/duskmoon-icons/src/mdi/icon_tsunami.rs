#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Tsunami)]
pub fn r#icon_tsunami(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M18.67 17.63C14.87 20.43 12.55 18.03 12 17.63C11.34 18.12 9.08 20.39 5.33 17.63C3.43 19.03 2.65 19 2 19V21C3.16 21 4.3 20.68 5.33 20.07C7.39 21.29 9.94 21.29 12 20.07C14.06 21.29 16.61 21.29 18.67 20.07C19.7 20.68 20.84 21 22 21V19C21.34 19 20.5 19 18.67 17.63M19.33 12H22V10H19.33C17.5 10 16 8.5 16 6.67C16 5.65 16.38 4.93 17.09 3.33C15.72 3.12 15.09 3 14 3C7.36 3 2.15 8.03 2 14.5L2 16.5C3.16 16.5 4.3 16.18 5.33 15.57C7.39 16.79 9.94 16.79 12 15.57C14.06 16.79 16.61 16.79 18.67 15.57C19.7 16.18 20.84 16.5 22 16.5V14.5C21.34 14.5 20.5 14.5 18.67 13.13C14.87 15.93 12.55 13.53 12 13.13C11.1 13.8 11.46 13.54 11.09 13.76C10.39 12.82 10 11.7 10 10.5C10 7.92 11.77 5.76 14.21 5.17C14.08 5.68 14 6.19 14 6.67C14 9.61 16.39 12 19.33 12Z" />
    </svg>
  }
}
