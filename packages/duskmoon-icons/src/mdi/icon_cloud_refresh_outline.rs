#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_CloudRefreshOutline)]
pub fn r#icon_cloud_refresh_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 18.5C12 19 12.07 19.5 12.18 20H6.5C5 20 3.69 19.5 2.61 18.43C1.54 17.38 1 16.09 1 14.58C1 13.28 1.39 12.12 2.17 11.1S4 9.43 5.25 9.15C5.67 7.62 6.5 6.38 7.75 5.43S10.42 4 12 4C13.95 4 15.6 4.68 16.96 6.04C18.32 7.4 19 9.05 19 11C20.15 11.13 21.1 11.63 21.86 12.5C22.1 12.76 22.29 13.05 22.46 13.36C21.36 12.5 20 12 18.5 12C18 12 17.5 12.07 17 12.18V11C17 9.62 16.5 8.44 15.54 7.46C14.56 6.5 13.38 6 12 6S9.44 6.5 8.46 7.46C7.5 8.44 7 9.62 7 11H6.5C5.53 11 4.71 11.34 4.03 12.03C3.34 12.71 3 13.53 3 14.5S3.34 16.29 4.03 17C4.71 17.66 5.53 18 6.5 18H12.03C12 18.17 12 18.33 12 18.5M18 14.5C15.79 14.5 14 16.29 14 18.5S15.79 22.5 18 22.5C19.68 22.5 21.12 21.47 21.71 20H20C19.54 20.61 18.82 21 18 21C16.62 21 15.5 19.88 15.5 18.5S16.62 16 18 16C18.69 16 19.32 16.28 19.77 16.73L18 18.5H22V14.5L20.83 15.67C20.11 14.95 19.11 14.5 18 14.5Z" />
    </svg>
  }
}
