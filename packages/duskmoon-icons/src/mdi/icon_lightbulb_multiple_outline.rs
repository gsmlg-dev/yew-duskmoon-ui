#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_LightbulbMultipleOutline)]
pub fn r#icon_lightbulb_multiple_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 21C12 21.55 12.45 22 13 22H15C15.55 22 16 21.55 16 21V20H12M14 7C11.24 7 9 9.24 9 12C9 13.57 9.74 15.06 11 16V18C11 18.55 11.45 19 12 19H16C16.55 19 17 18.55 17 18V16C19.21 14.34 19.66 11.21 18 9C17.06 7.74 15.57 7 14 7M15 14.82V17H13V14.82C11.44 14.27 10.62 12.55 11.17 11C11.72 9.43 13.44 8.61 15 9.16C16.56 9.72 17.38 11.43 16.83 13C16.53 13.85 15.85 14.5 15 14.82M7.68 15H7V16C7 16.55 7.45 17 8 17H9V16.88C8.46 16.33 8 15.7 7.68 15M13.6 5C12.5 2.47 9.53 1.33 7 2.45S3.34 6.5 4.45 9.04C4.79 9.81 5.33 10.5 6 11V13C6 13.55 6.45 14 7 14H7.3C7.1 13.35 7 12.68 7 12C7 8.29 9.89 5.21 13.6 5Z" />
    </svg>
  }
}
