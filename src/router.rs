use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::home::Main;
use crate::components::about::About;
use crate::components::projects::Projects;
use crate::components::contact::ContactPage;
use crate::components::pagenotfound::Page404;


#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Routes {
    #[at("/")]
    Main,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/projects")]
    Projects,
    #[at("/~")]
    Root,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Routes) -> Html {
  match route {
    Routes::Main => html! { <Main/> },
    Routes::About => html! { <About/> },
    Routes::Projects => html! { <Projects/> },
    Routes::Contact => html! { <ContactPage/> },
    Routes::Root => todo!(),
    Routes::NotFound => html! { <Page404/> },
  }
}
