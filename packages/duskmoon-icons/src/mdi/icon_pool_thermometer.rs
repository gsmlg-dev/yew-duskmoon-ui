#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_PoolThermometer)]
pub fn r#icon_pool_thermometer(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 6C17.9 6 17 6.9 17 8V14.8C16.4 15.4 16 16.2 16 17C16 18.7 17.3 20 19 20S22 18.7 22 17C22 16.1 21.6 15.3 21 14.8V8C21 6.9 20.1 6 19 6M19 7C19.5 7 20 7.4 20 8V9H18V8C18 7.4 18.5 7 19 7M15 20.3C14.1 20.7 13.2 21 12.3 21C10.1 21 7.9 19 5.6 19C4.4 19 3.2 19.3 1.9 19.7V17.7C3.2 17.3 4.4 17 5.7 17C7.9 17 10.1 19 12.4 19C13.3 19 14.2 18.7 15.1 18.3V20.3M12.3 17C13.2 17 14.1 16.7 15 16.3V14.3C14.3 14.6 13.7 14.8 13 15V5C13 4.4 13.4 4 14 4H16.8C16.4 2.8 15.3 2 14 2C12.3 2 11 3.3 11 5V6H6V5C6 4.4 6.4 4 7 4H9.8C9.4 2.8 8.3 2 7 2C5.3 2 4 3.3 4 5V13.2C3.3 13.3 2.7 13.5 2 13.8V15.8C3.2 15.3 4.4 15 5.7 15C7.9 15 10.1 17 12.3 17M6 8H11V10H6V8M6 12H11V14.8C9.3 14.3 7.7 13.2 6 13V12Z" />
    </svg>
  }
}
