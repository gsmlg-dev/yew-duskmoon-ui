#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_AccountTieVoiceOutline)]
pub fn r#icon_account_tie_voice_outline(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M16.75 4.36C18.77 6.56 18.77 9.61 16.75 11.63L15.07 9.94C15.91 8.76 15.91 7.23 15.07 6.05L16.75 4.36M20.06 1C24 5.05 23.96 11.11 20.06 15L18.43 13.37C21.2 10.19 21.2 5.65 18.43 2.63L20.06 1M13.36 13.76C15.31 14.42 17 15.5 17 17V22H1V17C1 15.5 2.69 14.42 4.65 13.76L5.27 15L5.5 15.5C4 15.96 2.9 16.62 2.9 17V20.1H7.12L8 15.03L7.06 13.15C7.68 13.08 8.33 13.03 9 13.03S10.32 13.08 10.94 13.15L10 15.03L10.88 20.1H15.1V17C15.1 16.62 14 15.96 12.5 15.5L12.73 15L13.36 13.76M9 6C7.9 6 7 6.9 7 8S7.9 10 9 10 11 9.1 11 8 10.1 6 9 6M9 12C6.79 12 5 10.21 5 8S6.79 4 9 4 13 5.79 13 8 11.21 12 9 12Z" />
    </svg>
  }
}
