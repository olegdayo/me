use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
struct TalkProps {
    title: AttrValue,
    tags: AttrValue,
    description: AttrValue,
    #[prop_or_default]
    children: Children,
}

#[function_component(Talk)]
fn talk(props: &TalkProps) -> Html {
    html! {
        <article class="talk-card">
            <h2>{&props.title}</h2>
            <p class="talk-tags">{&props.tags}</p>
            <p>{&props.description}</p>
            <div class="talk-resources">{for props.children.iter()}</div>
        </article>
    }
}

#[function_component(Talks)]
pub fn talks() -> Html {
    html! {
        <div class="general talks-page">
            <nav class="back-row" aria-label="Back navigation">
                <Link<Route> to={Route::General}>{"← Back"}</Link<Route>>
            </nav>
            <header>
                <h1>{"Talks"}</h1>
                <p>{"Conference talks, recordings, and articles."}</p>
                <a href="https://github.com/olegdayo/talks">{"View all materials on GitHub"}</a>
            </header>
            <section class="talk-list" aria-label="Talks">
                <Talk title="Contextual Talk" tags="Go" description="We will explore the history and internals of the context package as well as best practices of working with it.">
                    <a class="talk-thumbnail" href="https://youtu.be/heXa5PnY200">
                        <img alt="Watch Contextual Talk (Russian)" src="https://img.youtube.com/vi/heXa5PnY200/maxresdefault.jpg" />
                    </a>
                    <a href="https://youtu.be/heXa5PnY200">{"Watch recording (Russian)"}</a>
                    <a href="https://github.com/olegdayo/talks/tree/master/contextual-talk">{"Materials"}</a>
                </Talk>
                <Talk title="Obscure Data Structures of Go STL and where to Use Them" tags="Go" description="We will explore unpopular data structures of both Go standard library and golang.org/x (experimental packages).">
                    <a href="https://ecode.ozon.tech/archive/2024/talks/b14777a889154860ac8b580aea73ffe8">{"Watch recording (Russian)"}</a>
                </Talk>
                <Talk title="Container under the Hood or how 4 SysCalls Reshaped the Approaches to Operate Software" tags="Go · UNIX/Linux" description="We will explore the history and internals of UNIX/Linux containers, and how to implement one with Go.">
                    <a href="https://drive.google.com/file/d/12XRe6Y8sv7XfbMExBvm-UOjAb4VBFNdU/view">{"Watch recording (Russian)"}</a>
                    <a href="https://docs.google.com/presentation/d/1f0TDzx-3mSe1kNnYMi8lezfpNnqrTE_HysMUIejFiI4/edit?usp=drivesdk">{"Slides"}</a>
                    <a href="https://github.com/olegdayo/talks/tree/master/containers">{"Code & materials"}</a>
                </Talk>
            </section>
            <section aria-labelledby="papers-heading">
                <h2 id="papers-heading">{"Papers"}</h2>
                <Talk title="Container under the Hood or how 4 SysCalls Reshaped the Approaches to Operate Software" tags="Go · UNIX/Linux" description="We will explore the history and internals of UNIX/Linux containers.">
                    <a class="talk-thumbnail" href="https://habr.com/p/1004422">
                        <img alt="Read Container under the Hood or how 4 SysCalls Reshaped the Approaches to Operate Software (Russian)" src="https://habrastorage.org/r/w1560/getpro/habr/upload_files/1cb/d05/aca/1cbd05aca1dc01ad4cafb6abd040eaf4.png" />
                    </a>
                    <a href="https://habr.com/p/1004422">{"Read article (Russian)"}</a>
                </Talk>
            </section>
        </div>
    }
}
