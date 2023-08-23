use crate::UserContext;
use crate::UserInfo;
use crate::components::util;

use gloo_net::http::Request;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Contact {
  service: String,
  image: String,
  title: String,
  link: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Contacts {
  data: Vec<Contact>,
}

#[derive(Properties, PartialEq)]
struct ContactProp {
    contacts: UseStateHandle<Contacts>,
}

#[function_component(InfoInstance)]
fn info_instance(props: &ContactProp) -> Html {
  let mut contacts: Vec<yew::virtual_dom::VNode> = vec![];

  for c in props.contacts.data.clone() {
    let html: yew::virtual_dom::VNode = html! {
      <div class="flex justify-center mt-6">
        <div class="mr-2">
          {c.service} {":"}
        </div>
        <a href={c.link} target="_blank" class="items-center justify-center flex group-hover:underline underline-offset-2">
          <button>
            {c.title}
          </button>
        </a>
      </div>
    };
    contacts.push(html);
  }
  html! {
  <div>
    {for contacts}
  </div>
  }
}

#[function_component(ContactPage)]
pub fn app() -> Html {
  let user_context: UseReducerHandle<UserInfo> = use_context::<UserContext>().unwrap();
  let dark_mode: bool = user_context.dark_mode;
  use_effect(move || {
    let info: UserInfo = UserInfo {
      page: "Contact".to_string(),
      dark_mode: user_context.dark_mode.to_owned(),
      toggle_dropdown: user_context.toggle_dropdown.to_owned()
    };
    user_context.dispatch(info);
  });

  // Fetch contact information from Github
  let contacts: UseStateHandle<Contacts> = use_state(|| Contacts { data: vec![] });
  let contacts_val: UseStateHandle<Contacts> = contacts.clone();
  {
  use_effect_with_deps(move |_| {
    let contacts: UseStateHandle<Contacts> = contacts.clone();
    wasm_bindgen_futures::spawn_local(async move {
      let fetched_contacts: Vec<Contact> = Request::get("https://raw.githubusercontent.com/EJX537/portfolio_data/main/contacts.json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
      let contacts_data = Contacts {
        data: fetched_contacts,
      };
      contacts.set(contacts_data);
     });
    || ()
    }, ());
  }

  html! {
    <div class="flex flex-1">
      <div class="flex flex-1 flex-col sm:flex-row">

        <div class={classes!(util::util::either!(dark_mode == true => "hover:bg-dg-Bright_Gray"; "hover:bg-lg-Bright_Gray"), "contact-box", "group", "mb-2", "hover:shadow-xl")}>
          <div class="flex justify-center group-hover:font-bold group-hover:underline underline-offset-4">
            {"Peronsal Contact"}
          </div>
          <InfoInstance contacts={contacts_val}/>
        </div>

        <div class={classes!(util::util::either!(dark_mode == true => "hover:bg-dg-Bright_Gray"; "hover:bg-lg-Bright_Gray"), "contact-box", "group", "mb-2", "hover:shadow-xl")}>
          <div class="flex justify-center group-hover:font-bold group-hover:underline underline-offset-4">
            {"Social Media"}
          </div>
          <div class="flex justify-center mt-6">
            {"Do Not Use"}
          </div>

        </div>
      </div>
    </div>
  }
}



