use crate::UserContext;
use crate::UserInfo;
use crate::router::Routes;
use crate::components::project_display::ProjectDisplay;
use crate::components::util;

use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use std::collections::HashMap;


#[derive(Properties, PartialEq)]
pub struct Props {
  pub path: String
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Project {
  pub project_display: String,
  pub project_nav: String,
  pub preview_images: Vec<String>
}

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectList {
  pub projects: Vec<Project>
}

#[derive(Properties, PartialEq)]
pub struct ButtonListProps {
  pub projects: UseStateHandle<ProjectList>,
  pub on_mouse_over: Callback<MouseEvent>,
  pub on_mouse_out: Callback<MouseEvent>,
  dark_mode: bool
}

#[derive(Properties, PartialEq, Clone)]
struct PreviewMap {
  state: HashMap<String, Vec<String>>
}


#[function_component(ButtonList)]
fn button_list(props: &ButtonListProps) -> Html{
  let dark_mode: bool = props.dark_mode.clone();
  let project: UseStateHandle<ProjectList> = props.projects.clone();
  let projects: Vec<Project> = project.projects.clone();
  let mut projects_html: Vec<yew::virtual_dom::VNode> = vec![];
  
  projects_html.push(html! {
    <li class=" text-center font-bold py-4 text-xl relative">
      {"Projects"}
      <span class="absolute right-0 mr-4">
        {projects.len()}
      </span>
    </li>
  });


  for project in projects {
    let html = html! {
      <li class="py-4 group" onmouseleave={props.on_mouse_out.clone()} onmouseenter={props.on_mouse_over.clone()} id={project.project_display.clone()}>
        <Link<Routes> to={Routes::Projects{project: project.project_nav}} classes="justify-center ml-auto mr-2">
          <button>
            <span class="font-bold hidden group-hover:inline transition-all ease-in-out duration-1000 transform -translate-x-full hover:translate-x-0">{"-> "}</span>
            {project.project_display}
          </button>
        </Link<Routes>>
      </li>
    };
    projects_html.push(html);
  }

  html! {
    <ul class={classes!(util::util::either!(dark_mode == true => "divide-lg-Cultured"; "divide-dg-Cultured"), "w-full", "md:w-4/5", "h-4/5", "p-4", "rounded-lg", "divide-y-2", "shadow-lg")}>
      { for projects_html }
    </ul>
 }
}

#[function_component(Projects)]
pub fn app(props: &Props) -> Html {
  let user_context: UseReducerHandle<crate::UserInfo> = use_context::<UserContext>().unwrap();
  let user_context_clone: UseReducerHandle<UserInfo> = user_context.clone();
  let image_preview: UseStateHandle<PreviewMap> = use_state(move || PreviewMap{
    state: HashMap::new()
  });
  let image_preview_clone: UseStateHandle<PreviewMap> = image_preview.clone();
  let image_selected: UseStateHandle<usize> = use_state(move || {0});
  let project: String = props.path.clone();
  
  use_effect(move || {
    let info: UserInfo = UserInfo {
      page: "Projects".to_string(),
      dark_mode: user_context.dark_mode.to_owned(),
      toggle_dropdown: user_context.toggle_dropdown.to_owned()
    };
    user_context.dispatch(info);
  });
  // Fetch information from Github
  let projects: UseStateHandle<ProjectList> = use_state(|| {
    ProjectList {
      projects: vec![Project{ project_display: "Portifolio Website V2".to_string(), project_nav: "portfolio_v2".to_string(), preview_images: vec![] }]
    }
  });

  let projects_clone: UseStateHandle<ProjectList> = projects.clone();
  use_effect_with_deps(move |_| {
    let projects: UseStateHandle<ProjectList> = projects.clone();

    wasm_bindgen_futures::spawn_local(async move {
      match Request::get("https://assets.ericjxie.com/projects/projects.json").send().await {
        Ok(response) => {
          if response.ok() {
            let fetched_projects = response
            .json()
            .await
            .unwrap();
            let projects_data: ProjectList = ProjectList {
              projects: fetched_projects,
            };
            projects.set(projects_data.clone());
            let mut image_preview_clone: HashMap<String, Vec<String>> = image_preview.state.clone();
            for project in projects_data.projects {
              image_preview_clone.insert(project.project_display, project.preview_images);
            }
            image_preview.set(PreviewMap { state: image_preview_clone });
          }
        }
          Err(_error) => {}
      }
    });
    || ()
  }, ());

  // On hover change the image as a preview
  let hover_image: UseStateHandle<String> = use_state(|| {
    "".to_string()
  });
  let hover_image_clone: UseStateHandle<String> = hover_image.clone();
  let hover_image_val: String = hover_image.clone().to_string();

  let on_mouse_over: Callback<MouseEvent> = Callback::from(move |event: MouseEvent| {
    let target: web_sys::EventTarget = event.target().unwrap();
    let element: web_sys::HtmlElement = target.dyn_into::<web_sys::HtmlElement>().unwrap();
    let id: String = element.id();

    let map: HashMap<String, Vec<String>> = image_preview_clone.state.clone();
    
    if let Some(vec) = map.get(&id) {
      hover_image.set(vec[*image_selected].clone());
    }
  });

  let on_mouse_out: Callback<MouseEvent> = Callback::from(move |_event: MouseEvent| {
    hover_image_clone.set("".to_string());
  });

  html! {
    <div class="flex-1 flex flex-row h-full w-full">

      <div class="w-3/5 h-full relative hidden md:flex">
        <div class="w-[90%] h-[75%] bottom-0 left-0 absolute pr-2 pt-2 rounded-r-xl overflow-hidden shadow-xl">
          <img src={hover_image_val.clone()} alt="" class={classes!(util::util::either!(hover_image_val == "" => "hidden"; "block"), "w-full", "h-full", "object-contain", "bg-lg-Cultured", "rounded-r-xl")}/>
        </div>
      </div>

      <div class="flex w-4/5 md:w-2/5 items-center ml-auto">
        <ButtonList projects={projects_clone.clone()} on_mouse_over={on_mouse_over} on_mouse_out={on_mouse_out} dark_mode={user_context_clone.dark_mode} />
      </div>

      <ProjectDisplay path={project} projects={projects_clone}/>

    </div>
  }
}