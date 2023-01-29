#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_FoodHotDog)]
pub fn r#icon_food_hot_dog(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 5.77C20.85 5.65 20.72 5.55 20.59 5.45L20.62 5.41C21.4 4.63 21.4 3.37 20.62 2.59C19.84 1.81 18.58 1.81 17.79 2.59L17.05 3.33C15.68 2.3 13.74 2.4 12.5 3.65L3.65 12.5C2.4 13.74 2.3 15.68 3.33 17.05L2.59 17.79C1.8 18.58 1.8 19.84 2.59 20.62C3.37 21.4 4.63 21.4 5.41 20.62L5.45 20.59C5.55 20.72 5.65 20.85 5.77 21C7.13 22.34 9.35 22.34 10.72 21L20.97 10.72C22.34 9.35 22.34 7.14 21 5.77M4.77 15.61C4.5 15.05 4.6 14.36 5.06 13.9L13.9 5.06C14.36 4.6 15.05 4.5 15.61 4.77L4.77 15.61M19.56 9.3L9.3 19.56C8.72 20.15 7.77 20.15 7.18 19.56C6.6 19 6.6 18 7.18 17.44L17.44 7.18C18 6.6 19 6.6 19.56 7.18C20.15 7.77 20.15 8.72 19.56 9.3Z" />
    </svg>
  }
}