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
