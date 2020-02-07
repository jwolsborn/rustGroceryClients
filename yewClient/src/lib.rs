#![recursion_limit = "128"]
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender, services::fetch::*};
use yew::format::{Json, Nothing};
use yew::services::Task;
use failure::Error;

pub struct Model {
    link: ComponentLink<Self>,
    fetch_service: FetchService,
    value: String,
    fetching: bool,
}

pub enum Msg {
    FetchData,
    FetchReady(Result<String, Error>),
    GotInput(String),
    Clicked,
    Add,
    Ignore,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            fetch_service: FetchService::new(),
            link,
            value: "".into(),
            fetching: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.value = new_value;
            }
            Msg::Clicked => {
                self.value = "blah blah blah".to_string();
            }

            Msg::Add => {
                self.fetching = true; // 4.

                let callback = self.link.callback(
                    move |response: Response<Json<Result<String, Error>>>| { // 2.
                        let (meta, Json(data)) = response.into_parts();
                        if meta.status.is_success() {
                            Msg::FetchReady(data)
                        } else {
                            Msg::Ignore
                        }
                    },
                );

                let request = Request::post("127.0.0.1/add/apples").body(Nothing).unwrap();
                let task = self.fetch_service.fetch(request,callback);
            }
            _ => {
                true;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <textarea rows=5
                        value=&self.value
                        oninput=self.link.callback(|e: InputData| Msg::GotInput(e.value))
                        placeholder="Type item to add or remove">
                    </textarea>
                    <button onclick=self.link.callback(|_| Msg::Add)>{ "Add Item" }</button>
                    <button onclick=self.link.callback(|_| Msg::Clicked)>{ "Remove Item" }</button>
                </div>
                <div>
                    {&self.value}
                </div>
            </div>
        }
    }
}