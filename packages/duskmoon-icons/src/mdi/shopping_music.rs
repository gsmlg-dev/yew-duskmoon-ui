#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_ShoppingMusic)]
pub fn r#icon_shopping_music(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12,3A3,3 0 0,0 9,6H15A3,3 0 0,0 12,3M19,6A2,2 0 0,1 21,8V20A2,2 0 0,1 19,22H5C3.89,22 3,21.1 3,20V8C3,6.89 3.89,6 5,6H7A5,5 0 0,1 12,1A5,5 0 0,1 17,6H19M9,19L16.5,14L9,10V19Z" />
    </svg>
  }
}
