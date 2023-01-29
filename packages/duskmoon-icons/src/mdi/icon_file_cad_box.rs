#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_FileCadBox)]
pub fn r#icon_file_cad_box(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M5 3C3.9 3 3 3.9 3 5V19C3 20.1 3.9 21 5 21H19C20.1 21 21 20.1 21 19V5C21 3.9 20.1 3 19 3M11.25 5.25H12.75V6.38C13.58 6.38 14.25 7.05 14.25 7.88V10.37L14.11 10.5L15.18 12.36C15.55 11.76 15.75 11.07 15.75 10.36H17.25C17.26 11.61 16.81 12.82 16 13.77L18 17.25V18.75L16.7 18L14.84 14.78C13.12 15.91 10.89 15.91 9.16 14.78L7.3 18L6 18.75V17.25L9.89 10.5L9.75 10.37V7.88C9.75 7.05 10.42 6.38 11.25 6.38M12 7.88C11.16 7.88 10.74 8.9 11.34 9.5C11.94 10.08 12.95 9.65 12.94 8.81C12.94 8.29 12.5 7.88 12 7.88M11 11.6L9.91 13.5C11.17 14.36 12.83 14.36 14.09 13.5L13 11.6C12.43 12.11 11.57 12.11 11 11.6Z" />
    </svg>
  }
}
