#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_EarbudsOffOutline)]
pub fn r#icon_earbuds_off_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M22.11 21.46L18.5 17.85V17.84L15.58 14.93L15.58 14.93L14.09 13.44L14.1 13.44L11.56 10.9L11.56 10.9L10 9.34V9.34L7 6.34L7 6.34L6.66 6L6.67 6L6.1 5.44L2.39 1.73L1.11 3L2.55 4.43C2.19 5 2 5.58 2 6V11C2 12 3 14 5 14C5.61 14 6.32 13.72 7 13.27V20C7 20.55 7.45 21 8 21H9C9.55 21 10 20.55 10 20V11.89L14.5 16.39V20C14.5 20.55 14.95 21 15.5 21H17.5C17.94 21 18.3 20.72 18.43 20.32L20.84 22.73L22.11 21.46M8 10.23L5.91 11.6C5.4 11.93 5.08 12 5 12C4.3 12 4 11.08 4 11L4 6.03C4 6 4 5.97 4.03 5.92L8 9.89V10.23M13.53 10.33L11.04 7.84C11.37 5.12 13.69 3 16.5 3C19.54 3 22 5.46 22 8.5C22 11.32 19.89 13.63 17.16 13.96L14.67 11.47C15.2 11.8 15.83 12 16.5 12C18.43 12 20 10.43 20 8.5S18.43 5 16.5 5 13 6.57 13 8.5C13 9.17 13.2 9.8 13.53 10.33Z" />
    </svg>
  }
}
