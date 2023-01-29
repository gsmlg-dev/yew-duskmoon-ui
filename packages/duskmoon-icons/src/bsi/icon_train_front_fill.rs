#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(BS_TrainFrontFill)]
pub fn r#icon_train_front_fill(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M10.621.515C8.647.02 7.353.02 5.38.515c-.924.23-1.982.766-2.78 1.22C1.566 2.322 1 3.432 1 4.582V13.5A2.5 2.5 0 0 0 3.5 16h9a2.5 2.5 0 0 0 2.5-2.5V4.583c0-1.15-.565-2.26-1.6-2.849-.797-.453-1.855-.988-2.779-1.22ZM6.5 2h3a.5.5 0 0 1 0 1h-3a.5.5 0 0 1 0-1Zm-2 2h7A1.5 1.5 0 0 1 13 5.5v2A1.5 1.5 0 0 1 11.5 9h-7A1.5 1.5 0 0 1 3 7.5v-2A1.5 1.5 0 0 1 4.5 4Zm.5 9a1 1 0 1 1-2 0 1 1 0 0 1 2 0Zm0 0a1 1 0 1 1 2 0 1 1 0 0 1-2 0Zm8 0a1 1 0 1 1-2 0 1 1 0 0 1 2 0Zm-3-1a1 1 0 1 1 0 2 1 1 0 0 1 0-2ZM4 5.5a.5.5 0 0 1 .5-.5h3v3h-3a.5.5 0 0 1-.5-.5v-2ZM8.5 8V5h3a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-.5.5h-3Z"/>
    </svg>
  }
}
