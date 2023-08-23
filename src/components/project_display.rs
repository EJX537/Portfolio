use crate::UserContext;
use crate::UserInfo;
use crate::components::util;
use crate::router::Routes;

use wasm_bindgen::JsCast;
use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use yew_router::prelude::*;

use super::projects::ProjectList;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub projects: UseStateHandle<ProjectList>,
  pub path: String
}

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub struct ProjectInstance {
  pub title: String,
  pub link: String,
  pub images: Vec<String>,
  pub body: String
}

#[derive(Debug, Clone, PartialEq, Default, Properties)]
struct ProjectDisplayInfo {
  project: ProjectInstance,
  image: i32
}

#[derive(Properties, PartialEq)]
struct ImageProps {
  projectinfo: UseStateHandle<ProjectDisplayInfo>
}

#[function_component(ImageList)]
fn image_list(props: &ImageProps) -> Html{
  let project = props.projectinfo.clone();
  let project_clone = project.clone();
  let image_list = project.project.images.clone();
  let image_list_length = image_list.len();
  let current_image = project.image as usize;
  let mut img_path: &String = &"".to_string();

  let onclick_toggle_page: Callback<MouseEvent> = Callback::from(move |event: MouseEvent| {
    let target: web_sys::EventTarget = event.target().unwrap();
    let element: web_sys::HtmlElement = target.dyn_into::<web_sys::HtmlElement>().unwrap();
    let id: String = element.id();

    let mut project_info = ProjectDisplayInfo {
      project: project.project.to_owned(),
      image: current_image as i32
    };

    if id == "left" {
      project_info.image = project_info.image - 1;
    } else if id == "right" {
      project_info.image = project_info.image + 1;
    }
    project.set(project_info);
  }); 

  if image_list_length > current_image {
    img_path = &image_list[current_image];
  };

  html! {
    <div class="h-1/2 relative">
      <button class={classes!(util::util::either!(project_clone.image == 0 => "hidden"; "flex"), "absolute", "top-0", "bottom-0", "left-0", "border", "border-black", "rounded-lg", "items-center", "pl-2", "bg-lg-Cultured", "opacity-5" ,"hover:opacity-30")} id="left" onclick={onclick_toggle_page.clone()}>
        <div class="w-4 h-4 transform rotate-45 -translate-x-2 -translate-y-2 border-b-4 border-l-4 border-black ml-2"/>
      </button>
      
      <img src={img_path.clone()} alt="" class="h-full object-center object-contain mx-auto bg-lg-Cultured"/>
      
      <button class={classes!(util::util::either!(project_clone.image == image_list_length as i32 - 1 => "hidden"; "flex"),"absolute", "top-0", "bottom-0", "right-0", "border", "border-black", "rounded-lg", "items-center", "pl-2", "bg-lg-Cultured", "opacity-5" ,"hover:opacity-30")} id="right" onclick={onclick_toggle_page}>
        <div class="w-4 h-4 transform rotate-45 -translate-x-2 -translate-y-2 border-t-4 border-r-4 border-black ml-2" />
      </button>
    </div>
  }
}

#[function_component(ProjectDisplay)]
pub fn project_display(props: &Props) -> Html{
  let user_context: UseReducerHandle<UserInfo> = use_context::<UserContext>().unwrap();
  let dark_mode: bool = user_context.dark_mode;
  let mut projects: Vec<String> = vec![];
  let path: String = props.path.clone();
  let path_clone: String = path.clone();
  let path_clone2: String = path.clone();
  let path_clone3: String = path.clone();
  let navigator: Navigator = use_navigator().unwrap();

  for project in props.projects.projects.clone() {
    projects.push(project.project_nav);
  }
  // Makes sure that on-load the page the user is on exist otherwise it routes them to the project main page
  use_effect(move || {
    if projects.contains(&path) {
    } else {
    navigator.push(&Routes::Projects{project: "~".to_string()});
    }
  });


  // The project that is currently being displayed
  let display_project: UseStateHandle<ProjectDisplayInfo> = use_state(|| {
    ProjectDisplayInfo::default()
  });
  let display_project_clone: UseStateHandle<ProjectDisplayInfo> = display_project.clone();

  use_effect_with_deps(move |_| {
    if path_clone2 != "~".to_string() {
      let display_project: UseStateHandle<ProjectDisplayInfo> = display_project.clone();
        
        wasm_bindgen_futures::spawn_local(async move {
          let url = format!("https://raw.githubusercontent.com/EJX537/portfolio_data/main/projects/{}", path_clone2);
          match Request::get(&url).send().await {
            Ok(response) => {
              if response.ok() {
                let fetched_project:ProjectInstance = response
                .json()
                .await
                .unwrap();
              display_project.set(ProjectDisplayInfo { project: fetched_project, image: 0 });
            }
          }
          Err(_error) => {
            display_project.set(ProjectDisplayInfo::default());
          }
        }
      });
    } else {
      // On close the information is flushed
      display_project.set(ProjectDisplayInfo::default());
    }
    || ()
  }, path_clone3);

  html! {
    <div class={classes!(util::util::either!(path_clone == "~" => "hidden"; "flex"), util::util::either!(dark_mode == true => "bg-dg-Cultured"; "bg-lg-Cultured"), "absolute", "md:w-full", "h-screen", "top-0", "block", "bg-opacity-75", "w-screen")}>
      <div class={classes!(util::util::either!(dark_mode == true => "bg-dg-Cultured"; "bg-lg-Cultured"), "flex", "flex-1", "md:m-8", "m-1", "border-2", "border-black", "z-50", "flex-col", "rounded-lg")}>
        <div class={"flex justify-center items-center m-2 h-[5%] relative"}>
          <div class="flex h-full items-center ml-auto">
            <a href={display_project_clone.project.link.clone()} target="_blank" class="h-full">
              <button class="h-full">
                <img src="https://raw.githubusercontent.com/EJX537/portfolio_data/main/images/github-mark.png" alt=""
                  class={classes!(util::util::either!(dark_mode == true => "invert"; "invert-0"), "h-[90%]", "mr-2")}/>
              </button>
            </a>
            {display_project_clone.project.title.clone()}
          </div>
          <Link<Routes> to={Routes::Projects{project: "~".to_string()}} classes="justify-center ml-auto mr-2 h-full items-center flex">
            <button class=" font-bold flex h-full items-center">
              <div class="relative w-10 h-10 m-2">
                <div class="absolute inset-0 flex items-center justify-center rotate-45 ">
                  <div class="w-1 h-full bg-black absolute"></div>
                  <div class="h-1 w-full bg-black absolute"></div>
                </div>
              </div>

            </button>
          </Link<Routes>>

        </div>
        <div class="flex flex-col h-[95%] w-full p-2">
          <ImageList projectinfo={display_project_clone.clone()}/>
          <div class="h-1/2 p-2 pt-4 overflow-hidden text-ellipsis text-sm indent-4">
            {display_project_clone.project.body.clone()}
          </div>
        </div>
      </div>
    </div>
  }
}




