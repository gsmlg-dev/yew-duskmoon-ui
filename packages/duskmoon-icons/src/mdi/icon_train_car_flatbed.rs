#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TrainCarFlatbed)]
pub fn r#icon_train_car_flatbed(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M23 15V17H22C22 18.11 21.11 19 20 19S18 18.11 18 17H6C6 18.11 5.11 19 4 19S2 18.11 2 17H1V15H23Z" />
    </svg>
  }
}
