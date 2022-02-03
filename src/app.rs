use crate::routes::{counter::Counter, home::Home, AppRoute};
use crate::components::{ navbar::Navbar, footer::Footer, };


use yew::prelude::*;
use yew_router::prelude::*;

pub struct Main;

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Navbar/>
                <main class="main">
                    <Router<AppRoute, ()>
                        render = Router::render(|switch: AppRoute| {
                            match switch {
                                AppRoute::Home => html!{ <Home/> },
                                AppRoute::Counter => html!{ <Counter/> },
                            }
                        })
                    />
                </main>
                <Footer/>
            </>
        }
    }
}
