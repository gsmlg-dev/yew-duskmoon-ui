#![allow(non_camel_case_types)]

use yew::prelude::*;
use self::props::IconProps;

#[function_component(MD_Snowshoeing)]
pub fn r#icon_snowshoeing(props: &IconProps) -> Html {
  let owned_props = props.clone();

  html! {
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" id={owned_props.id} class={owned_props.class} width={owned_props.size} fill={owned_props.color} style={owned_props.style}>
      <path d="M12.5 3.5C12.5 2.4 13.4 1.5 14.5 1.5S16.5 2.4 16.5 3.5 15.6 5.5 14.5 5.5 12.5 4.6 12.5 3.5M6.32 19.03L5.18 17.56L4 18.5L6.38 21.54C6.89 22.19 7.54 22.69 8.26 22.95C8.54 23.05 8.79 23 9 22.84C9.28 22.61 9.4 22.14 9.1 21.77C9 21.67 8.9 21.6 8.79 21.55C8.36 21.37 7.97 21.1 7.65 20.72L7.57 20.62L11 18.2L11.89 15L14 17V21.5H12V23H15.87C16.69 23 17.5 22.79 18.13 22.39C18.39 22.23 18.5 22 18.5 21.75C18.5 21.37 18.2 21 17.73 21C17.6 21 17.47 21.04 17.36 21.1C16.96 21.33 16.5 21.47 16 21.5V15.5L13.89 13.5L14.5 10.5C15.79 12 17.8 13 20 13V11C18.1 11 16.5 10 15.69 8.58L14.69 7C14.29 6.4 13.69 6 13 6C12.24 6 11.58 6.34 7 8.28V13H9V9.58L10.79 8.88L9.2 17L6.32 19.03Z" />
    </svg>
  }
}
