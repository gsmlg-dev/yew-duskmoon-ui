#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_CloudPrintOutline)]
pub fn r#icon_cloud_print_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M15 15H9V14H15V15M15 16H9V17H15V16M15 18H9V19H15V18M23 13.5C23 14.75 22.56 15.81 21.69 16.69C20.81 17.56 19.75 18 18.5 18H18V22H6V17.95C4.7 17.85 3.57 17.36 2.61 16.43C1.54 15.38 1 14.09 1 12.58C1 11.28 1.39 10.12 2.17 9.1S4 7.43 5.25 7.15C5.67 5.62 6.5 4.38 7.75 3.43S10.42 2 12 2C13.95 2 15.6 2.68 16.96 4.04C18.32 5.4 19 7.05 19 9C20.15 9.13 21.1 9.63 21.86 10.5C22.62 11.35 23 12.35 23 13.5M6 15.95V11H17V9C17 7.62 16.5 6.44 15.54 5.46C14.56 4.5 13.38 4 12 4S9.44 4.5 8.46 5.46C7.5 6.44 7 7.62 7 9H6.5C5.53 9 4.71 9.34 4.03 10.03C3.34 10.71 3 11.53 3 12.5S3.34 14.29 4.03 15C4.59 15.54 5.25 15.85 6 15.95M16 13H8V20H16V13M21 13.5C21 12.8 20.76 12.21 20.27 11.73S19.2 11 18.5 11H18V16H18.5C19.2 16 19.79 15.76 20.27 15.28S21 14.2 21 13.5Z" />
    </svg>
  }
}
