use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
use self::pages::*;

#[derive(yew_router::Switch, Clone)]
pub enum MainRoute {
    #[to = "/blog"]
    Blog,
    #[to = "/"]
    Home,
}

pub struct Main;

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <header class="navbar">
                    <ul>
                        <li>
                            <RouterAnchor<MainRoute> route=MainRoute::Home><a>{"Home"}</a></RouterAnchor<MainRoute>>
                        </li>
                        <li>
                            <RouterAnchor<MainRoute> route=MainRoute::Blog><a>{"Blog"}</a></RouterAnchor<MainRoute>>
                        </li>
                    </ul>
                </header>
                <div class="main">
                    <Router<MainRoute, ()>
                        render = Router::render(|switch: MainRoute| {
                            match switch {
                                MainRoute::Home => html!{ <Home/> },
                                MainRoute::Blog => html!{ <Blog/> },
                            }
                        })
                    />
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Main>();
}
