#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_HeatPump)]
pub fn r#icon_heat_pump(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 3H5C3.9 3 3 3.9 3 5V19C3 20.1 3.9 21 5 21H19C20.1 21 21 20.1 21 19V5C21 3.9 20.1 3 19 3M12.75 7.08C13.57 7.2 14.32 7.5 14.95 8L12.75 10.19V7.08M11.25 7.08V10.19L9.05 8C9.68 7.5 10.43 7.2 11.25 7.08M8 9.05L10.19 11.25H7.08C7.2 10.43 7.5 9.68 8 9.05M7.08 12.75H10.19L8 14.95C7.5 14.32 7.2 13.57 7.08 12.75M11.25 16.92C10.43 16.8 9.68 16.5 9.05 16L11.25 13.81V16.92M12 13C11.45 13 11 12.55 11 12S11.45 11 12 11 13 11.45 13 12 12.55 13 12 13M12.75 16.92V13.81L14.95 16C14.32 16.5 13.57 16.8 12.75 16.92M16 14.95L13.81 12.75H16.92C16.8 13.57 16.5 14.32 16 14.95M13.81 11.25L16 9.05C16.5 9.69 16.8 10.44 16.92 11.25H13.81Z" />
    </svg>
  }
}
