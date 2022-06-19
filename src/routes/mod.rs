pub mod home;
pub mod problems;
pub mod problem_template;

use yew_router::prelude::*;
use yew::prelude::*;

use home::Home;
use problems::*;
use crate::solutions::Solutions;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/problems")]
    Problems,

    #[at("/problems/:id")]
    Problem { id: Solutions},
    
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },

        Route::Problems => html! {
            <Problems />
        },

        Route::Problem { id } => html! {
            <Problem id={(*id).clone()} />
        },

        Route::NotFound => html! {
            <h1>{ "404" }</h1>
        },
    }
}
