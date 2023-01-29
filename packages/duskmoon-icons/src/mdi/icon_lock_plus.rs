#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_LockPlus)]
pub fn r#icon_lock_plus(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M19 13C19.34 13 19.67 13.04 20 13.09V10C20 8.9 19.11 8 18 8H17V6C17 3.24 14.76 1 12 1S7 3.24 7 6V8H6C4.9 8 4 8.89 4 10V20C4 21.11 4.89 22 6 22H13.81C13.3 21.12 13 20.1 13 19C13 15.69 15.69 13 19 13M9 6C9 4.34 10.34 3 12 3S15 4.34 15 6V8H9V6M12 17C10.9 17 10 16.11 10 15S10.9 13 12 13C13.1 13 14 13.89 14 15C14 16.11 13.11 17 12 17M23 18V20H20V23H18V20H15V18H18V15H20V18H23Z" />
    </svg>
  }
}
