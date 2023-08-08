use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Routes {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/~")]
    Root,
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
  match route {
    Route::Home => todo!(),
    Route::About => todo!(),
    Route::Contact => todo!(),
    Route::Root => todo!(),
    Route::NotFound => todo!(),
  }
}