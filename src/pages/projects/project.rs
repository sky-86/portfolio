use yew::prelude::*;

use super::project_list::ProjectEnum;

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectRouteProps {
    pub id: Option<ProjectEnum>,
}

// renders the correct project page based on the id
#[function_component(ProjectRoute)]
pub fn project_route(props: &ProjectRouteProps) -> Html {
    let id = match &props.id {
        Some(id) => id,
        None => return html! {
            <h1>{"Error: project does not exist"}</h1>
        }
    };

    let props = match id {
        ProjectEnum::SortingVisual => {
            ProjectProps {
                id: id.clone(),
                name: "Sorting Visualizer".to_string(),
                about: "This is the about section".to_string(),
                github: "https://github.com".to_string(),
                _instruct: Some("This is the intructions".to_string()),
                _url: Some("/projects/sorting_visual".to_string()),
            }
        },
        ProjectEnum::MediaPlayer => {
            ProjectProps {
                id: id.clone(),
                name: "Media Player".to_string(),
                about: "This is the about section".to_string(),
                github: "https://github.com".to_string(),
                _instruct: Some("This is the intructions".to_string()),
                _url: Some("/projects/media_player".to_string()),
            }
        }
        ProjectEnum::Portfolio => {
            ProjectProps {
                id: id.clone(),
                name: "Portfolio".to_string(),
                about: "This is the about section".to_string(),
                github: "https://github.com".to_string(),
                _instruct: None, 
                _url: None,
            }
        },
        ProjectEnum::Contacts => {
            ProjectProps {
                id: id.clone(),
                name: "Contacts App".to_string(),
                about: "This is the about section".to_string(),
                github: "https://github.com".to_string(),
                _instruct: None, 
                _url: Some("https://contacts.up.railway.app".into()),
            }
        }
    };
    html! {<Project ..props />}
}

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectProps {
    id: ProjectEnum,
    name: String,
    about: String,
    github: String,
    // bug... dead code false positive; #[allow[dead_code]] not working
    _instruct: Option<String>,
    _url: Option<String>,
}

// the project info template
#[function_component(Project)]
pub fn project(props: &ProjectProps) -> Html {
    html! {
        <>
        <ul>
            <li><h1>{&props.name}</h1></li>
            <li>{&props.id}</li>
            <li>{&props.about}</li>
            <li><a href={props.github.clone()}>{"Github"}</a></li>
            if let Some(i) = &props._instruct {
                <li>{i}</li>
            }

            if let Some(d) = &props._url {
                <li><a href={d.clone()}>{"Project Link"}</a></li>
            }
        </ul>
        </>
    }
}
