#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FacebookMessenger)]
pub fn r#icon_facebook_messenger(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,2C6.36,2 2,6.13 2,11.7C2,14.61 3.19,17.14 5.14,18.87C5.3,19 5.4,19.22 5.41,19.44L5.46,21.22C5.5,21.79 6.07,22.16 6.59,21.93L8.57,21.06C8.74,21 8.93,20.97 9.1,21C10,21.27 11,21.4 12,21.4C17.64,21.4 22,17.27 22,11.7C22,6.13 17.64,2 12,2M18,9.46L15.07,14.13C14.6,14.86 13.6,15.05 12.9,14.5L10.56,12.77C10.35,12.61 10.05,12.61 9.84,12.77L6.68,15.17C6.26,15.5 5.71,15 6,14.54L8.93,9.87C9.4,9.14 10.4,8.95 11.1,9.47L13.44,11.23C13.66,11.39 13.95,11.39 14.16,11.23L17.32,8.83C17.74,8.5 18.29,9 18,9.46Z" />
    </svg>
  }
}
