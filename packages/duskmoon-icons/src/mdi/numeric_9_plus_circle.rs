#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_Numeric9PlusCircle)]
pub fn r#icon_numeric_9_plus_circle(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19,11H17V9H15V11H13V13H15V15H17V13H19V11M10,7H8A2,2 0 0,0 6,9V11C6,12.11 6.9,13 8,13H10V15H6V17H10A2,2 0 0,0 12,15V9C12,7.89 11.1,7 10,7M8,9H10V11H8V9M12,2A10,10 0 0,1 22,12A10,10 0 0,1 12,22A10,10 0 0,1 2,12A10,10 0 0,1 12,2Z" />
    </svg>
  }
}
