use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;
use crate::pages::page_not_found::NotFound;
use crate::pages::problems::{problem::*, problem_list::*};
use crate::pages::projects::{project::*, project_list::*};

// All possible routes in app
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/problems/")]
    Problems,

    #[at("/problems/:id/")]
    Problem { id: ProblemEnum},

    #[at("/projects/")]
    Projects,

    #[at("/projects/:id/")]
    Project { id: ProjectEnum },

    #[not_found]
    #[at("/404/")]
    NotFound,
}

// is called when route is changed; renders route
pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },

        Route::Problems => html! {
            <ProblemList />
        },

        Route::Problem { id } => html! {
            <ProblemRoute id={id.clone()} />
        },

        Route::Projects => html! {
            <ProjectList />
        },

        Route::Project { id } => html! {
            <ProjectRoute id={id.clone()} />
        },

        Route::NotFound => html! {
            <NotFound />
        },
    }
}
