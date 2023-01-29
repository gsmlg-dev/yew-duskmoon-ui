#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CloudClockOutline)]
pub fn r#icon_cloud_clock_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M23 15.5C23 14.79 22.84 14.14 22.55 13.54C21.89 11.79 20.56 10.38 18.86 9.62C18.61 8.26 18 7.06 16.96 6.04C15.6 4.68 13.95 4 12 4C10.42 4 9 4.47 7.75 5.43S5.67 7.62 5.25 9.15C4 9.43 2.96 10.08 2.17 11.1S1 13.28 1 14.58C1 16.09 1.54 17.38 2.61 18.43C3.69 19.5 5 20 6.5 20H10.26C11.53 21.81 13.62 23 16 23C19.87 23 23 19.87 23 16C23 15.89 23 15.79 23 15.68C23 15.62 23 15.56 23 15.5M6.5 18C5.53 18 4.71 17.66 4.03 17C3.34 16.29 3 15.47 3 14.5S3.34 12.71 4.03 12.03C4.71 11.34 5.53 11 6.5 11H7C7 9.62 7.5 8.44 8.46 7.46C9.44 6.5 10.62 6 12 6S14.56 6.5 15.54 7.46C16 7.93 16.35 8.46 16.59 9.03C16.4 9 16.2 9 16 9C12.13 9 9 12.13 9 16C9 16.7 9.11 17.37 9.29 18H6.5M16 21C13.24 21 11 18.76 11 16S13.24 11 16 11 21 13.24 21 16 18.76 21 16 21M16.5 16.25L19.36 17.94L18.61 19.16L15 17V12H16.5V16.25Z" />
    </svg>
  }
}
