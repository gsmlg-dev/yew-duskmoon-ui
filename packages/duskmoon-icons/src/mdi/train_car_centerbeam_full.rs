#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::icon_props::IconProps;

#[function_component(MD_TrainCarCenterbeamFull)]
pub fn r#icon_train_car_centerbeam_full(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M23 6V17H22C22 18.11 21.11 19 20 19S18 18.11 18 17H6C6 18.11 5.11 19 4 19S2 18.11 2 17H1V6H3V15H21V6H23M8 12H4V14H8V12M15 12H9V14H15V12M20 12H16V14H20V12M8 9H4V11H8V9M15 9H9V11H15V9M20 9H16V11H20V9M8 6H4V8H8V6M15 6H9V8H15V6M20 6H16V8H20V6Z" />
    </svg>
  }
}
