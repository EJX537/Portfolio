use crate::UserContext;
use crate::UserInfo;
use crate::components::util;
use crate::router::Routes;

use wasm_bindgen::JsCast;
use yew::prelude::*;
use gloo_net::http::Request;
use gloo_console::log;
use serde::Deserialize;
use yew::functional::use_reducer;
use std::rc::Rc;
use yew_router::prelude::*;

use super::projects::ProjectInstance;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub projects: Vec<String>,
  pub path: String
}

#[function_component(ProjectDisplay)]
pub fn project_display(props: &Props) -> Html{

  let sample_project: ProjectInstance = ProjectInstance {
    image: "".to_string(),
    title: "Portfolio Website V2".to_string(),
    body: "Rust based web assembly site powered by Yew and Tailwind.".to_string(),
    link: "https://github.com/EJX537/Portfolio".to_string()
  };

  let projects: Vec<String> = props.projects.clone();
  let path: String = props.path.clone();
  let path_clone: String = path.clone();
  let navigator: Navigator = use_navigator().unwrap();
  let nav_clone: Navigator = navigator.clone();
  use_effect(move || {
    if projects.contains(&path) {

    } else {
    navigator.push(&Routes::Projects{project: "~".to_string()});
    }
  });

  let onclick_nav_return: Callback<MouseEvent> = Callback::from(move |_event: MouseEvent| {
    log!(1);
    nav_clone.push(&Routes::Projects{project: "~".to_string()});
  });

  html! {
    <div class={classes!(util::util::either!(path_clone == "~" => "hidden"; "flex"), "absolute", "md:w-full", "h-full", "block", "bg-opacity-75", "bg-lg-Cultured", "w-screen")}>
      <div class="flex flex-1 md:m-8 m-1 border-2 border-black z-50 bg-lg-Cultured flex-col rounded-lg op">
        <div class={"flex justify-center items-center m-2 p-2 h-[5%] relative"}>
          <div class="flex h-full items-center ml-auto">
            <a href={sample_project.link} target="_blank" class="h-full">
              <button class="h-full">
                <img src="https://raw.githubusercontent.com/EJX537/portfolio_data/main/images/github-mark.png" alt="" class="h-[90%] mr-2"/>
              </button>
            </a>
            {sample_project.title}
          </div>
          <Link<Routes> to={Routes::Projects{project: "~".to_string()}} classes="justify-center ml-auto mr-2">
          <button class="" onclick={onclick_nav_return}>
            {"EXIT"}
          </button>
          </Link<Routes>>

        </div>
        <div class="flex flex-col h-[95%] w-full p-2">
          <div class="h-1/2">
          <img src="https://raw.githubusercontent.com/EJX537/portfolio_data/9d59fdd4183d253488d65ccced028f779cae526c/images/Linkedin.png" alt="PLACEHOLDER" class="h-full w-full object-center object-contain"/>
          </div>
          <div class="h-1/2 p-2 pt-4 overflow-hidden text-ellipsis text-sm indent-4">
            {sample_project.body}
          </div>
        </div>
      </div>
    </div>
  }
}




