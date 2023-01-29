#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_SnowflakeCheck)]
pub fn r#icon_snowflake_check(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M17.75 21.16L15 18.16L16.16 17L17.75 18.59L21.34 15L22.5 16.41L17.75 21.16M12 18C12 16.46 12.59 15.06 13.54 14L12 14.89L9.5 13.44V10.56L12 9.11L14.5 10.56V13.13C15.08 12.71 15.75 12.4 16.46 12.21V10.56L18.46 9.43L20.79 10.05L21.31 8.12L19.54 7.65L20 5.88L18.07 5.36L17.45 7.69L15.45 8.82L13 7.38V5.12L14.71 3.41L13.29 2L12 3.29L10.71 2L9.29 3.41L11 5.12V7.38L8.5 8.82L6.5 7.69L5.92 5.36L4 5.88L4.47 7.65L2.7 8.12L3.22 10.05L5.55 9.43L7.55 10.56V13.45L5.55 14.58L3.22 13.96L2.7 15.89L4.47 16.36L4 18.12L5.93 18.64L6.55 16.31L8.55 15.18L11 16.62V18.88L9.29 20.59L10.71 22L12 20.71L13.29 22L13.42 21.87C12.54 20.83 12 19.5 12 18Z" />
    </svg>
  }
}
