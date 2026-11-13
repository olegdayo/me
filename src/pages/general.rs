use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(General)]
pub fn general() -> Html {
    html! {
        <div class="general landing">
            <div class="landing-content">
                <div class="landing-intro">
                <img class="landing-photo" src="assets/images/profile.png" alt="Oleg Sidorenkov" />
                <div class="landing-introduction">
                <h1>{"Hey there! The name's Oleg"}</h1>
                <section class="landing-summary" aria-labelledby="tldr-heading">
                    <h2 id="tldr-heading">{"TL; DR:"}</h2>
                    <ul>
                        <li>{"Senior SWE / Tech Lead at VK Cloud KMS"}</li>
                        <li>{"HSE (Higher School of Economics) MSc in Software Engineering"}</li>
                        <li>{"HSE Operating Systems course instructor"}</li>
                        <li>{"Open-source contributor: Vault, Terraform, K8s stuff"}</li>
                        <li>{"Public speaker at IT conferences, tech writer: Go, UNIX"}</li>
                        <li>{"Dev: Go, Rust, PostgreSQL, Kafka"}</li>
                        <li>{"Ops: Linux, FreeBSD, Shell, Kubernetes, Terraform, Vault"}</li>
                    </ul>
                </section>
                </div>
                </div>
                <nav class="page-links" aria-label="Main navigation">
                    <Link<Route> to={Route::CV}>
                        <img class="navigation-icon" src="assets/images/cv.png" alt="" />
                        <span>{"CV"}</span>
                    </Link<Route>>
                    <Link<Route> to={Route::Talks}>
                        <img class="navigation-icon" src="assets/images/microphone.png" alt="" />
                        <span>{"Talks"}</span>
                    </Link<Route>>
                    <a href="https://t.me/olegdayo">
                        <img class="navigation-icon" src="assets/images/telegram.png" alt="" />
                        <span>{"Chat"}</span>
                    </a>
                </nav>
            </div>
            <footer class="visitors">
                <img alt="Visitors' count" src="https://count.getloli.com/get/@olegdayo?theme=gelbooru" />
            </footer>
        </div>
    }
}
