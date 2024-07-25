use yew::prelude::*;

#[function_component(General)]
pub fn general() -> Html {
    html! {
        <>
            <div class="profile">
                <p> {"Hey there! The name's Oleg"} </p>
                <img class="profile" src={ "assets/images/profile.png" } alt="Profile" />
            </div>
        </>
    }
}

#[function_component(Profile)]
fn profile() -> Html {
    html! {}
}

#[function_component(WhoAmI)]
fn whoami() -> Html {
    html! {}
}

#[function_component(Contacts)]
fn contacts() -> Html {
    html! {}
}

#[function_component(Carrier)]
fn carrier() -> Html {
    html! {}
}

#[function_component(Education)]
fn education() -> Html {
    html! {}
}

#[function_component(AdditionalEducation)]
fn additional_education() -> Html {
    html! {}
}

#[function_component(Languages)]
fn loanguages() -> Html {
    html! {}
}

#[function_component(Skills)]
fn skills() -> Html {
    html! {}
}

#[function_component(OlimpiadsAndConferences)]
fn tech_stack() -> Html {
    html! {}
}

#[function_component(Hobbies)]
pub fn hobbies() -> Html {
    html! {}
}

#[function_component(VisitorsCount)]
fn visitors_count() -> Html {
    html! {
        <img alt={"Visitors' count"} src={"https://moe-counter.glitch.me/get/@offluck-web-page?theme=gelbooru"}/>
    }
}
