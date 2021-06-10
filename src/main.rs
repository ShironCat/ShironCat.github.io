use yew::prelude::*;

#[derive(Debug)]
pub struct Model;
impl Component for Model {
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
            <main>
                <img class="profile-picture" src="images/avatar.jpg" alt="ShironCat's avatar" />
                <h1>{ "Hello, World!" }</h1>
            </main>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
