#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_LightbulbFluorescentTubeOutline)]
pub fn r#icon_lightbulb_fluorescent_tube_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M7.05 2.81L6.5 3.34L5.11 1.92L3.87 3.16L5.28 4.58L4.58 5.28L3.16 3.87L1.92 5.11L3.34 6.5L2.81 7.05L4.22 8.46L15.54 19.78L16.95 21.19L17.5 20.66L18.89 22.08L20.13 20.84L18.72 19.43L19.43 18.72L20.84 20.13L22.08 18.89L20.66 17.5L21.19 16.95L19.78 15.54L8.46 4.22M7.05 5.64L18.36 16.95L16.95 18.36L5.64 7.05Z" />
    </svg>
  }
}
