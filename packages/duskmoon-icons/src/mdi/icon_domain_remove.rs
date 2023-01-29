#![allow(non_camel_case_types)]

use yew::prelude::*;
use super::props::IconProps;

#[function_component(MD_DomainRemove)]
pub fn r#icon_domain_remove(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12 7V3H2V21H13.35A5.8 5.8 0 0 1 13 19H12V17H13.35A5 5 0 0 1 14 15.69V15H12V13H14V11H12V9H20V13.09A5.58 5.58 0 0 1 22 13.81V7M6 19H4V17H6M6 15H4V13H6M6 11H4V9H6M6 7H4V5H6M10 19H8V17H10M10 15H8V13H10M10 11H8V9H10M10 7H8V5H10M16 13H18V11H16M16 11V13H18V11M16 11V13H18V11M22.54 16.88L20.41 19L22.54 21.12L21.12 22.54L19 20.41L16.88 22.54L15.46 21.12L17.59 19L15.46 16.88L16.88 15.46L19 17.59L21.12 15.46Z" />
    </svg>
  }
}
