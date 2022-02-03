use yew::{prelude::*};
use js_sys::Date;
pub struct Counter { 
    value: i64,
    link: ComponentLink<Self>
}

pub enum Msg { 
    Increment,
    Decrement
}
impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { 
            value: 0,
            link: _link
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg { 
            Msg::Increment => { 
                self.value +=1;
                true
            },
            Msg::Decrement => { 
                self.value -=1;
                true
            }
        }
    }
    /// Components may be rerendered by their parents
    /// When this happens, they could receive new properties and need to re-renders
    /// This design facilitates parent to child component communication by just changing the values of PROPERTY
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let increment = self.link.callback(move |_| Msg::Increment);
        let decrement = self.link.callback(move |_| Msg::Decrement);
       
        html! {
            <>
                <div>
                    <div class="panel">
                        <button class="button" onclick={increment}>{"Increment"}</button>
                        <button class="button" onclick={decrement}>{"decrement"}</button>
                    </div>
                    <p class="counter">
                        {self.value}
                    </p>
                    <p>{ "Rendered: " }
                    { String::from(Date::new_0().to_string()) }</p>
                </div>
            </>
        }
    }
}