#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_DatabaseRefresh)]
pub fn r#icon_database_refresh(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 3C16.42 3 20 4.79 20 7C20 9.21 16.42 11 12 11C7.58 11 4 9.21 4 7C4 4.79 7.58 3 12 3M4 9C4 11.21 7.58 13 12 13C13.11 13 14.18 12.89 15.14 12.68C14.19 13.54 13.5 14.67 13.18 15.96L12 16C7.58 16 4 14.21 4 12V9M20 9V11L19.5 11L18.9 11.03C19.6 10.43 20 9.74 20 9M4 14C4 16.21 7.58 18 12 18L13 17.97C13.09 19.03 13.42 20 13.95 20.88L12 21C7.58 21 4 19.21 4 17V14M19 13.5C20.11 13.5 21.11 13.95 21.83 14.67L23 13.5V17.5H19L20.77 15.73C20.32 15.28 19.69 15 19 15C17.62 15 16.5 16.12 16.5 17.5C16.5 18.88 17.62 20 19 20C19.82 20 20.54 19.61 21 19H22.71C22.12 20.47 20.68 21.5 19 21.5C16.79 21.5 15 19.71 15 17.5C15 15.29 16.79 13.5 19 13.5Z" />
    </svg>
  }
}
