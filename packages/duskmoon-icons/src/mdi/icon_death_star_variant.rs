#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_DeathStarVariant)]
pub fn r#icon_death_star_variant(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M2.05,13H21.94C21.88,13.69 21.74,14.36 21.54,15H14V17H17V19H15V21H12.5V22C12.33,22 12.17,22 12,22C6.82,22 2.55,18.05 2.05,13M21.94,11H2.05C2.55,5.95 6.82,2 12,2C13.62,2 15.15,2.39 16.5,3.08V5H18.5V7H20V9H21.54C21.74,9.64 21.88,10.31 21.94,11M12,6.75A2.5,2.5 0 0,0 9.5,4.25A2.5,2.5 0 0,0 7,6.75A2.5,2.5 0 0,0 9.5,9.25A2.5,2.5 0 0,0 12,6.75Z" />
    </svg>
  }
}
