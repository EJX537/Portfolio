use yew::prelude::*;
use crate::UserContext;
use crate::UserInfo;
use crate::components::project_display::ProjectDisplay;

use gloo_console::log;
use gloo_net::http::Request;
use yew::functional::use_reducer;
use serde::Deserialize;



#[derive(Properties, PartialEq)]
pub struct Props {
  pub path: String
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ProjectInstance {
  pub image: String,
  pub title: String,
  pub body: String,
  pub link: String
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Project {
  pub project_display: String,
  pub project_nav: String,
  pub preview_images: Vec<String>
}

#[derive(Properties, PartialEq)]
pub struct ProjectList {
  pub projects: Vec<Project>
}


#[function_component(ButtonList)]
fn button_list(props: &ProjectList) -> Html{
  let projects: Vec<Project> = props.projects.clone();
  
  

  html! {
 }
}
≤  

#[function_component(Projects)]
pub fn app(props: &Props) -> Html {
  let user_context: UseReducerHandle<crate::UserInfo> = use_context::<UserContext>().unwrap();
  let project: String = props.path.clone();

  use_effect(move || {
    let info: UserInfo = UserInfo {
      page: "Projects".to_string(),
      dark_mode: user_context.clone().dark_mode.to_owned(),
    };
    user_context.dispatch(info);
  });

  // Fetch information
  let projects: UseStateHandle<ProjectList> = use_state(|| {
    ProjectList {
      projects: vec![Project{ project_display: "Portifolio Website V2".to_string(), project_nav: "portfolio_v2".to_string(), preview_images: vec![] }]
    }
  });

  use_effect_with_deps(move |_| {

    let projects: UseStateHandle<ProjectList> = projects.clone();

    wasm_bindgen_futures::spawn_local(async move {
      let fetched_projects: Vec<Project> = Request::get("https://raw.githubusercontent.com/EJX537/portfolio_data/main/projects/projects.json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

      let projects_data: ProjectList = ProjectList {
        projects: fetched_projects,
      };
      projects.set(projects_data);
      });
   
    || ()
  }, ());

  html! {
    <div class="flex-1 flex flex-row h-full w-full relative">

      <div class="w-3/5 h-full relative rounded-r-lg rounded-b-lg overflow-hidden hidden md:flex">
        <img src="https://raw.githubusercontent.com/EJX537/portfolio_data/9d59fdd4183d253488d65ccced028f779cae526c/images/Linkedin.png" alt="" class="p-2 w-[90%] h-[75%] object-contain absolute bottom-0 left-0"/>
      </div>

      <div class="flex w-4/5 md:w-2/5 items-center ml-auto">
        <ul class="w-full md:w-4/5 h-4/5 bg-lg-Cultured p-4 rounded-lg divide-y-2 divide-black shadow-lg ">
        <li class=" text-center font-bold py-4 text-xl relative">
          {"Projects"}
          <span class="absolute right-0 mr-4">
            {"0"}
          </span>
        </li>
        <li class="py-4 group">
          <button>
            <span class="font-bold hidden group-hover:inline transition-all ease-in-out duration-1000 transform -translate-x-full hover:translate-x-0">{"-> "}</span>
            {"Portifolio website"}
          </button>
        </li>
        <li class="py-4 group">
          <button>
            <span class="font-bold hidden group-hover:inline transition-all ease-in-out duration-1000 transform -translate-x-full hover:translate-x-0">{"-> "}</span>
            {"HTTP Server"}
          </button>
        </li>
        </ul>
      </div>

      <ProjectDisplay path={project} projects={vec!["portfolio_v2".to_string()]}/>

    </div>
  }
}