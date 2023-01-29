#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Qi)]
pub fn r#icon_qi(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12.09,6.56V14.64C12.09,14.72 12,14.79 11.94,14.79C9.62,14.79 7.75,12.91 7.75,10.6C7.75,8.28 9.62,6.4 11.94,6.4C12,6.4 12.09,6.47 12.09,6.56M12.09,17.86V20.29C12.06,21.69 13.12,22.87 14.5,23C14.66,23 14.78,22.89 14.79,22.73V4.47C14.78,4.3 14.68,4.15 14.5,4.09C10.92,2.67 6.86,4.44 5.44,8.03C4,11.63 5.79,15.69 9.38,17.11C10.16,17.42 11,17.58 11.81,17.6C11.95,17.59 12.07,17.71 12.08,17.87M16.19,5.5V15.72C16.19,16 16.19,16.2 16.46,16V16C19.42,13.5 19.81,9.09 17.33,6.13C17.07,5.81 16.77,5.5 16.46,5.26C16.19,5.05 16.19,5.26 16.19,5.53M14.79,2.43C14.79,1.66 15.42,1.03 16.19,1.03C16.96,1.03 17.59,1.66 17.59,2.43C17.59,3.2 16.96,3.83 16.19,3.83V3.83C15.42,3.83 14.79,3.2 14.79,2.43" />
    </svg>
  }
}
