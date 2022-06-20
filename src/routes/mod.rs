pub mod home;
pub mod problems;
pub mod problem_template;
pub mod about;
pub mod projects;

use yew_router::prelude::*;
use yew::prelude::*;

use home::Home;
use about::About;
use projects::Projects;
use problems::*;
use crate::solutions::Solutions;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/skyler")]
    Skyler,

    #[at("/about")]
    About,

    #[at("/problems")]
    Problems,

    #[at("/problems/:id")]
    Problem { id: Solutions},

    #[at("/projects")]
    Projects,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },

        Route::Skyler => html! {
            <Home />
        },


        Route::About => html! {
            <About />
        },

        Route::Problems => html! {
            <Problems />
        },

        Route::Problem { id } => html! {
            <Problem id={(*id).clone()} />
        },

        Route::Projects => html! {
            <Projects />
        },

        Route::NotFound => html! {
            <h1>{ "404" }</h1>
        },
    }
}
