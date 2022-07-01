use yew::prelude::*;
use yew_router::prelude::*;
use strum_macros::EnumString;
use std::fmt;

use crate::route::Route;

// all included projects
#[derive(Clone, Debug, PartialEq, EnumString)]
pub enum ProjectEnum {
    SortingVisual,
    MediaPlayer,
    Portfolio,
}

// converts enum name to string,
// need for displaying the url
impl fmt::Display for ProjectEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}
 
#[function_component(ProjectList)]
pub fn project_list() -> Html {
    html! {
        <div>
            <h1>{ "Projects" }</h1>
            <ul>
                <li><Link<Route> to={Route::Project{id: ProjectEnum::SortingVisual}}>
                    {"Sorting Visualizer"}
                </Link<Route>></li>
                <li><Link<Route> to={Route::Project{id: ProjectEnum::MediaPlayer}}>
                   {"Media Player"} 
                </Link<Route>></li>
                <li><Link<Route> to={Route::Project{id: ProjectEnum::Portfolio}}>
                   {"Portfolio"} 
                </Link<Route>></li>
             </ul>
        </div>
    }
}
