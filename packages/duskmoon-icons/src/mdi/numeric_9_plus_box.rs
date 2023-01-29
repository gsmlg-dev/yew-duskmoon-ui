#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Numeric9PlusBox)]
pub fn r#icon_numeric_9_plus_box(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21,5V19A2,2 0 0,1 19,21H5A2,2 0 0,1 3,19V5A2,2 0 0,1 5,3H19A2,2 0 0,1 21,5M19,11H17V9H15V11H13V13H15V15H17V13H19V11M10,7H8A2,2 0 0,0 6,9V11C6,12.11 6.9,13 8,13H10V15H6V17H10A2,2 0 0,0 12,15V9C12,7.89 11.1,7 10,7M8,9H10V11H8V9Z" />
    </svg>
  }
}
