use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(General)]
pub fn general() -> Html {
    html! {
        <div class="general landing">
            <div class="landing-content">
                <h1>{"Hey there! The name's Oleg"}</h1>
                <img class="landing-photo" src="assets/images/profile.png" alt="Oleg Sidorenkov" />
                <section class="landing-summary" aria-labelledby="tldr-heading">
                    <h2 id="tldr-heading">{"TL; DR:"}</h2>
                    <ul>
                        <li>{"Senior SWE / Tech Lead at VK Cloud KMS"}</li>
                        <li>{"HSE BSc + MSc SE alumni"}</li>
                        <li>{"HSE OS instructor"}</li>
                        <li>{"OSS contributor"}</li>
                        <li>{"Dev: Go; Rust; PostgreSQL; Kafka"}</li>
                        <li>{"Ops: Linux; FreeBSD; Shell; Kubernetes; Terraform; Vault"}</li>
                    </ul>
                </section>
                <a class="contact-link" href="https://t.me/olegdayo">
                    <img src="assets/images/telegram.png" alt="" width="32" height="32" />
                    <span>{"Contact me"}</span>
                </a>
                <nav class="page-links" aria-label="Main navigation">
                    <Link<Route> to={Route::CV}>
                        <img class="navigation-icon" src="assets/images/cv.png" alt="" />
                        <span>{"CV"}</span>
                    </Link<Route>>
                    <Link<Route> to={Route::Talks}>
                        <img class="navigation-icon" src="assets/images/microphone.png" alt="" />
                        <span>{"Talks"}</span>
                    </Link<Route>>
                </nav>
            </div>
            <footer class="visitors">
                <img alt="Visitors' count" src="https://count.getloli.com/get/@olegdayo?theme=gelbooru" />
            </footer>
        </div>
    }
}
