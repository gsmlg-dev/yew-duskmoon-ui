#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_TranslateOff)]
pub fn r#icon_translate_off(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12.17,5.81C11.87,6.69 11.47,7.55 11,8.39L12.35,9.74C13.11,8.5 13.71,7.18 14.13,5.81H17.16V3.75H9.94V1.69H7.87V3.75H6.37L8.43,5.81H12.17M15.53,12.91L17.03,14.41L17.67,12.69L19.08,16.47L22.39,19.77L18.7,9.94H16.64L15.53,12.91M1.31,1.31L0,2.62L1.13,3.75H0.65V5.81H3.19L5.26,7.88H4.46C5.21,9.56 6.24,11.15 7.53,12.58L2.28,17.76L3.75,19.22L8.91,14.07L12.11,17.27L12.8,15.43L14.1,16.72L12,22.31H14.06L15.22,19.22H16.6L21.38,24L22.69,22.69L1.31,1.31Z" />
    </svg>
  }
}
