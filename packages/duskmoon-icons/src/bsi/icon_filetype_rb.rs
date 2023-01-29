#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(BS_FiletypeRb)]
pub fn r#icon_filetype_rb(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path fill-rule="evenodd" d="M14 4.5V14a2 2 0 0 1-2 2H8v-1h4a1 1 0 0 0 1-1V4.5h-2A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v9H2V2a2 2 0 0 1 2-2h5.5L14 4.5ZM0 11.85h1.597c.297 0 .55.053.758.158.21.104.369.25.478.44.111.19.167.412.167.668a1.18 1.18 0 0 1-.727 1.122l.803 1.611h-.885l-.7-1.491H.782v1.491H0V11.85Zm.782.621v1.292h.689c.218 0 .391-.053.518-.158.13-.106.194-.264.194-.475 0-.213-.065-.376-.194-.489a.74.74 0 0 0-.507-.17h-.7Zm4.426 3.378H3.544V11.85h1.67c.357 0 .643.087.858.26.215.175.322.416.322.724a.94.94 0 0 1-.09.422.79.79 0 0 1-.244.293 1.002 1.002 0 0 1-.351.161v.035c.162.016.31.063.445.141a.846.846 0 0 1 .322.325.986.986 0 0 1 .123.51c0 .238-.06.441-.178.61-.12.167-.284.296-.492.386a1.85 1.85 0 0 1-.721.132Zm-.179-3.404h-.7v1.07h.521c.178 0 .323-.022.434-.065a.497.497 0 0 0 .249-.185.52.52 0 0 0 .082-.296.486.486 0 0 0-.155-.384c-.102-.093-.245-.14-.43-.14Zm.05 1.62h-.75v1.19h.589c.31 0 .534-.05.67-.147a.503.503 0 0 0 .206-.434.614.614 0 0 0-.082-.325.51.51 0 0 0-.24-.21.946.946 0 0 0-.393-.074Z"/>
    </svg>
  }
}
