use yew::prelude::*;
use crate::UserContext;
use crate::UserInfo;

use gloo_console::log;
use gloo_net::http::Request;
use yew::functional::use_reducer;
use serde::Deserialize;



#[derive(Properties, PartialEq)]
pub struct Props {
  pub path: String
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
struct PreviewProject {
  image: String,
  title: String,
  body: String,
  link: String
}


#[function_component(Projects)]
pub fn app(props: &Props) ->Html {
  let user_context: UseReducerHandle<crate::UserInfo> = use_context::<UserContext>().unwrap();
  use_effect(move || {
    let info: UserInfo = UserInfo {
      page: "Projects".to_string(),
      dark_mode: user_context.clone().dark_mode.to_owned(),
    };
    user_context.dispatch(info);
  });

  let sample_project: PreviewProject = PreviewProject {
    image: "".to_string(),
    title: "Portfolio Website V2".to_string(),
    body: "Power by Tailwind, Yew, and Tailwind".to_string(),
    link: "github.com".to_string()
  };


  let path: String = props.path.clone();

  html! {
    <div class="flex justify-center items-center flex-1">
      <div>
        <div>
          <img src={sample_project.image} alt="image preview" />
        </div>
        <div>
          <div>
            {sample_project.title}
          </div>
          <div>
            {sample_project.body}
          </div>
          <div>
            <a href="">
            <button>
            <img src="" alt="" />
            </button>
            </a>
          </div>
        </div>
      </div>
    </div>
  }
}