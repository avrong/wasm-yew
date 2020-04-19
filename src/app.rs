use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct App {
    clicked: bool,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true
            }
        }
        true
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Clicked!" } else { "Click me!" };

        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::Click)>{ button_text }</button>
            </div>
        }
    }
}