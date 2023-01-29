#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_Gog)]
pub fn r#icon_gog(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M4,3H20A2,2 0 0,1 22,5V19A2,2 0 0,1 20,21H4A2,2 0 0,1 2,19V5A2,2 0 0,1 4,3M10.46,5.54C9.95,5.54 9.54,5.95 9.54,6.46V9.54A0.92,0.92 0 0,0 10.46,10.46H13.54A0.92,0.92 0 0,0 14.46,9.54V6.46C14.46,5.95 14.05,5.54 13.54,5.54H10.46M11.08,6.77H12.92A0.31,0.31 0 0,1 13.23,7.08V8.92A0.31,0.31 0 0,1 12.92,9.23H11.08A0.31,0.31 0 0,1 10.77,8.92V7.08A0.31,0.31 0 0,1 11.08,6.77M4.92,5.54A0.92,0.92 0 0,0 4,6.46V9.54C4,10.05 4.41,10.46 4.92,10.46H7.08V9.23H5.54C5.37,9.23 5.23,9.09 5.23,8.92V7.08C5.23,6.91 5.37,6.77 5.54,6.77H7.38A0.31,0.31 0 0,1 7.69,7.08V10.77A0.31,0.31 0 0,1 7.38,11.08H4V12.31H8C8.5,12.31 8.92,11.89 8.92,11.38V6.46A0.92,0.92 0 0,0 8,5.54H4.92M16,5.54C15.5,5.54 15.08,5.95 15.08,6.46V9.54C15.08,10.05 15.5,10.46 16,10.46H18.15V9.23H16.62C16.45,9.23 16.31,9.09 16.31,8.92V7.08C16.31,6.91 16.45,6.77 16.62,6.77H18.46C18.63,6.77 18.77,6.91 18.77,7.08V10.77C18.77,10.94 18.63,11.08 18.46,11.08H15.08V12.31H19.08C19.59,12.31 20,11.89 20,11.38V6.46C20,5.95 19.59,5.54 19.08,5.54H16M9.85,13.54C9.34,13.54 8.92,13.95 8.92,14.46V17.54C8.92,18.05 9.34,18.46 9.85,18.46H12.92C13.43,18.46 13.85,18.05 13.85,17.54V14.46C13.85,13.95 13.43,13.54 12.92,13.54H9.85M10.46,14.77H12.31C12.5,14.77 12.62,14.91 12.62,15.08V16.92A0.31,0.31 0 0,1 12.31,17.23H10.46C10.29,17.23 10.15,17.09 10.15,16.92V15.08A0.31,0.31 0 0,1 10.46,14.77M4.92,13.54C4.41,13.54 4,13.95 4,14.46V17.54C4,18.05 4.41,18.46 4.92,18.46H8.31V17.23H5.54C5.37,17.23 5.23,17.09 5.23,16.92V15.08C5.23,14.91 5.37,14.77 5.54,14.77H8.31V13.54H4.92M15.38,13.54C14.87,13.54 14.46,13.95 14.46,14.46V18.46H15.69V15.08A0.31,0.31 0 0,1 16,14.77H16.62V18.46H17.85V14.77H18.77V18.46H20V13.54H15.38Z" />
    </svg>
  }
}
