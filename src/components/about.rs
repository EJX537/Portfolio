use crate::UserContext;
use crate::UserInfo;
use crate::components::util;

use wasm_bindgen::JsCast;
use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::functional::use_reducer;
use std::rc::Rc;


#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Skill {
  skill: String,
  image: String,
  placeholder: String,
  skill_type: Vec<String>,
  body: String,
}

#[derive(Debug, Clone, PartialEq)]
struct Skills {
  data: Vec<Skill>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Categories {
  categories: Vec<String>
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct Body {
  body: String,
  image: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdInfo {
    pub id: String,
}

#[derive(Properties, PartialEq)]
struct Props {
  window_size: UseStateHandle<util::WindowSize>,
  page: UseStateHandle<(isize, isize)>,
  skills: UseStateHandle<Skills>,
  skill_type: UseReducerHandle<IdInfo>,
  dark_mode: bool
}
impl Reducible for IdInfo {
  type Action = IdInfo;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    IdInfo { id: action.id}.into()
  }
}

#[function_component(SkillInstance)]
fn skill_instance(props: &Props) -> Html{
  let dark_mode: bool = props.dark_mode.clone();
  let height: u32 = props.window_size.height.clone();
  let width: u32 = props.window_size.width.clone();
  let page: UseStateHandle<(isize, isize)> = props.page.clone();
  let skills: Vec<Skill> = props.skills.data.clone();
  let skill_type: String = props.skill_type.id.clone();
  let mut skills_div: Vec<yew::virtual_dom::VNode> = vec![];
  // The amount of Skills to display
  let amount: isize;
  
  if width > 1535 {
    if height > 1299 {
      amount = 12;
    } else {
      amount = 6
    }
  } else if width > 1279 {
    if height > 999 {
      amount = 6;
    } else {
      amount = 3;
    }
  } else if width > 1023 {
    if height > 999 {
      amount = 6;
    } else {
      amount = 3;
    }
  } else if width > 767 {
    if height > 1199 {
      amount = 6;
    } else if height > 899 {
      amount = 4;
    } else {
      amount = 2
    }
  } else if width > 639 {
    if height > 999 {
      amount = 4;
    } else {
      amount = 2;
    }
  } else {
    if height > 999 {
      amount = 2
    } else {
      amount = 1;
    }
  }

  let mut count: isize = 0;
  for (_pos, e) in skills.iter().enumerate() {
    if e.clone().skill_type.contains(&skill_type) || skill_type == "___"{
      let page_temp: isize = (count / amount).try_into().unwrap();
      count += 1;
      let html: yew::virtual_dom::VNode = html! {
        <div class={classes!(util::util::either!(page_temp == page.0 => "block"; "hidden"),
          util::util::either!(dark_mode == true => "bg-dg-Bright_Gray hover:bg-dg-Gainsboro"; "bg-lg-Bright_Gray hover:bg-lg-Gainsboro"),
          "flex", "flex-col", "m-2", "p-2", "rounded-lg", "overflow-hidden", "group", "sm:max-h-[260px]", "2xl:max-h-max")}>
          <div class="justify-evenly flex group-hover:font-bold group-hover:underline underline-offset-4 items-end">
            <img src={e.clone().image} alt={e.clone().placeholder} class={classes!(util::util::either!(dark_mode == true => "invert"; "invert-0"), "h-[40px]", "w-[40px]", "object-cover")} />
              {e.clone().skill}
          </div>
          <div class="text-sm p-2">
            {e.clone().body}
          </div>
        </div>
      };
      skills_div.push(html);
    }
  }
  page.set((page.0, (count + amount - 1) / amount - 1));
  html! {
  <div class="flex-1 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-6 xl:m-4 justify-around">
    {for skills_div}
  </div>
  }
}

#[derive(Properties, PartialEq)]
struct DropdownProps {
  categories: UseStateHandle<Categories>,
  onclick_toggle_skill_menu: Callback<MouseEvent>,
  onclick_set_skill: Callback<MouseEvent>,
  skill_type: UseReducerHandle<IdInfo>,
  display_skill_menu: UseStateHandle<bool>,
  dark_mode: bool
}

#[function_component(DropdownMenu)]
fn dropdown_menu(props: &DropdownProps) -> Html{
  let categories: UseStateHandle<Categories> = props.categories.clone();
  let dark_mode: bool = props.dark_mode.clone();
  let skill_type: UseReducerHandle<IdInfo> = props.skill_type.clone();
  let skill_val: String = skill_type.id.to_owned();
  let onclick_toggle_skill_menu: Callback<MouseEvent> = props.onclick_toggle_skill_menu.clone();
  let onclick_set_skill: Callback<MouseEvent> = props.onclick_set_skill.clone();
  let display_skill_menu: UseStateHandle<bool> = props.display_skill_menu.clone();

  let mut div: Vec<yew::virtual_dom::VNode> = vec![];

  for c in categories.categories.clone() {
    let html: yew::virtual_dom::VNode = html! {
      <li class="py-1">
        <button id={c.clone()} onclick={onclick_set_skill.clone()}>
          {c}
        </button>
      </li>
    };
    div.push(html);
  }

  html! {
    <div class="flex justify-center font-bold my-2 sm:my-6">
      <div class="flex">
        {"My"} 
        <div class="relative">
        <button class={classes!(util::util::either!(dark_mode == true => "border-dg-Light_Gray bg-dg-Gainsboro"; "border-lg-Light_Gray bg-lg-Gainsboro"), "mx-2", "px-2", "border-2", "rounded-sm")} onclick={onclick_toggle_skill_menu}>
          {skill_val} {" \u{25BC}"}
        </button>
          <ul class={classes!(util::util::either!(*display_skill_menu == true => "block"; "hidden"), util::util::either!(dark_mode == true => "border-dg-Light_Gray bg-dg-Bright_Gray divide-lg-Cultured" ;"border-lg-Light_Gray bg-lg-Bright_Gray divide-dg-Cultured"), "absolute", "top-full", "left-0", "justify-start", "px-2", "ml-2", "border-2", "divide-y", "text-sm", "text-center", "z-50")}>
          <li class="py-1">
            <button id="___" onclick={onclick_set_skill.clone()}>
              {"***"}
            </button>
            </li>
            {for div}
          </ul>
        </div>
        {"Skills"}
      </div>
    </div>
  }
}


#[function_component(About)]
pub fn app() -> Html {
  let user_context: UseReducerHandle<UserInfo> = use_context::<UserContext>().unwrap();
  let user_context_clone = user_context.clone();
  let window_size: UseStateHandle<util::WindowSize> = util::use_window_size();
  use_effect(move || {
    let info: UserInfo = UserInfo {
      page: "About".to_string(),
      dark_mode: user_context.dark_mode.to_owned(),
      toggle_dropdown: user_context.toggle_dropdown.to_owned()
    };
    user_context.dispatch(info);
  });

  // 
  let pages: UseStateHandle<(isize, isize)> = use_state(|| (0, 0));
  let pages_clone: UseStateHandle<(isize, isize)> = pages.clone();
  let pages_clone2: UseStateHandle<(isize, isize)> = pages.clone();

  // Fetch information from website
  let skills: UseStateHandle<Skills> = use_state(|| Skills { data: vec![] });
  let skills_vals: UseStateHandle<Skills> = skills.clone();
  
  let skill_categories: UseStateHandle<Categories> = use_state(|| Categories{ categories: vec![] });
  let skill_categories_clone: UseStateHandle<Categories> = skill_categories.clone();
  // let body: UseStateHandle<Body> = use_state(|| Body { body: "".to_string(), image: "".to_string()});

  use_effect_with_deps(move |_| {
    let skills: UseStateHandle<Skills> = skills.clone();
    let skill_categories: UseStateHandle<Categories> = skill_categories.clone();

    wasm_bindgen_futures::spawn_local(async move {
      match Request::get("https://assets.ericjxie.com/skills/skills.json").send().await {
        Ok(response) => {
          if response.ok() {
            let fetched_skill: Vec<Skill> = response
            .json()
            .await
            .unwrap();
            skills.set(Skills{ data: fetched_skill })
          }
        }
        Err(_error) => {}
      }
    });


    wasm_bindgen_futures::spawn_local(async move {
      match Request::get("https://assets.ericjxie.com/skills/skill_categories.json").send().await {
        Ok(response) => {
          if response.ok() {
            let fetched_skill_categories: Vec<String> = response
            .json()
            .await
            .unwrap();
          skill_categories.set(Categories{ categories: fetched_skill_categories })
          }
        }
        Err(_error) => {}
      }
    });
    || ()
    }, ());

  // Display the dropdown menu
  let display_skill_menu: UseStateHandle<bool> = use_state(|| false);
  let display_skill_menu_clone: UseStateHandle<bool> = display_skill_menu.clone();
  let display_skill_menu_clone2: UseStateHandle<bool> = display_skill_menu.clone();
  
  // Set the skill to display
  let skill_type: UseReducerHandle<IdInfo> = use_reducer(|| IdInfo {
    id: "___".to_string()
  });
  let skill_type_clone: UseReducerHandle<IdInfo> = skill_type.clone();

  // Open the dropdown menu to select skill category
  let onclick_toggle_skill_menu: Callback<MouseEvent> = Callback::from(move |_event: MouseEvent| {
    display_skill_menu.set(!*display_skill_menu);
  }); 

  // Toggle whoch categories of skills to show
  let onclick_set_skill: Callback<MouseEvent> = Callback::from(move |event: MouseEvent| {
    let target: web_sys::EventTarget = event.target().unwrap();
    let element: web_sys::HtmlElement = target.dyn_into::<web_sys::HtmlElement>().unwrap();
    let id: String = element.id();
    let info: IdInfo = IdInfo {
      id
    };
    skill_type.dispatch(info);
    display_skill_menu_clone.set(false);
    pages_clone2.set((0, 0));
  });

  // To navigate to different pages of skills
  let onclick_toggle_page: Callback<MouseEvent> = Callback::from(move |event: MouseEvent| {
    let target: web_sys::EventTarget = event.target().unwrap();
    let element: web_sys::HtmlElement = target.dyn_into::<web_sys::HtmlElement>().unwrap();
    let id: String = element.id();
    if id == "left" {
      pages.set((pages.0 - 1, pages.1));
    } else if id == "right" {
      pages.set((pages.0 + 1, pages.1));
    }
  }); 

  html! {
    <div class="flex flex-1 flex-col h-full w-screen overflow-hidden">
      <div class="flex flex-col sm:flex-row sm:my-4 md:h-2/5 lg:h-2/5 xl:h-3/5 items-center justify-center">

        <div class="items-center justify-center sm:w-[40%] flex overflow-hidden w-full h-full">
          <img src="https://assets.ericjxie.com/images/image_self.png" alt="place holder" class="w-[90%] h-[150px] sm:h-full p-4 object-contain" />
        </div>

        <div class="flex p-4 sm:w-[55%] items-center justify-center leading-10 text-center sm:text-start">
          <div>
            <span>
              {"My name is Eric Xie and I am a 4th year Computer Engineering Student at the University of California, Santa Cruz. And here are my skills and "}
            </span>
            <a href={"https://assets.ericjxie.com/Resume.pdf"} target="_blank">
              <button class="text-2xl underline underline-offset-2 font-black">
                {"Resume"}
              </button>
            </a>
            <span>
              {"."}
            </span>
          </div>
        </div>

      </div>

      <div class="mt-auto">

        <DropdownMenu categories={skill_categories_clone} onclick_toggle_skill_menu={onclick_toggle_skill_menu} onclick_set_skill={onclick_set_skill} skill_type={skill_type_clone.clone()} display_skill_menu={display_skill_menu_clone2} dark_mode={user_context_clone.dark_mode} />

        <div class="flex flex-1 flex-col justify-center m-2 relative">
          <button class={classes!(util::util::either!(pages_clone.0 == 0 => "hidden"; "flex"), "absolute", "top-0", "bottom-0", "left-0", "border", "border-black", "rounded-lg", "items-center", "pl-2", "bg-lg-Cultured", "opacity-5" ,"hover:opacity-30")} onclick={onclick_toggle_page.clone()} id="left">
            <div class="w-4 h-4 transform rotate-45 -translate-x-2 -translate-y-2 border-b-4 border-l-4 border-black ml-2"/>
          </button>

          <SkillInstance window_size={window_size} page={pages_clone.clone()} skills={skills_vals} skill_type={skill_type_clone} dark_mode={user_context_clone.dark_mode}/>

          <button class={classes!(util::util::either!(pages_clone.0 == pages_clone.1 => "hidden"; "flex"), "absolute", "top-0", "bottom-0", "right-0", "border", "border-black", "rounded-lg", "items-center", "pl-2", "bg-lg-Cultured", "opacity-5" ,"hover:opacity-30")} onclick={onclick_toggle_page} id="right">
            <div class="w-4 h-4 transform rotate-45 -translate-x-2 -translate-y-2 border-t-4 border-r-4 border-black ml-2" />
          </button>

        </div>
      </div>
    </div>
  }
}



