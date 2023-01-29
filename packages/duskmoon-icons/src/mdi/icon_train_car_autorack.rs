#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_TrainCarAutorack)]
pub fn r#icon_train_car_autorack(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M21 6H3C1.9 6 1 6.9 1 8V17H2C2 18.11 2.9 19 4 19S6 18.11 6 17H18C18 18.11 18.9 19 20 19S22 18.11 22 17H23V8C23 6.9 22.11 6 21 6M7.58 12.18C8.19 12.18 8.68 12.68 8.68 13.29S8.19 14.39 7.58 14.39C6.97 14.39 6.47 13.9 6.47 13.29S6.97 12.18 7.58 12.18M7.03 11.08L8.68 9.61H11.63L14.58 11.08H7.03M9.66 14.03H14.34C14.5 14.42 14.74 14.75 15.05 15H8.95C9.26 14.75 9.5 14.42 9.66 14.03M15.32 13.29C15.32 12.68 15.81 12.18 16.42 12.18C17.03 12.18 17.53 12.68 17.53 13.29S17.03 14.39 16.42 14.39C15.81 14.4 15.32 13.9 15.32 13.29M17.79 15C18.11 14.75 18.36 14.42 18.5 14.03H20.11V13.29C20.11 12.47 19.35 12.21 18.63 11.82L12 8.5H8.32L6.11 10.34H5.37C4.55 10.34 3.89 11 3.89 11.82V14.03H5.5C5.64 14.42 5.89 14.75 6.21 15H3V8H21V15H17.79Z" />
    </svg>
  }
}
