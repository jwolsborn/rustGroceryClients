  #![recursion_limit = "256"]
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender, services::fetch::*};
use yew::format::{Json, Nothing};
use yew::services::Task;
use failure::Error;

  pub struct Model {
      link: ComponentLink<Self>,
      fetch_service: FetchService,
      value: String,
      grocery_list: Vec<String>,
      fetching: bool
  }

pub enum Msg {
    FetchData,
    FetchReady(Result<String, Error>),
    GotInput(String),
    Clicked,
    Add,
    Ignore,
    Remove,
    GetList,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            fetch_service: FetchService::new(),
            link,
            grocery_list: Vec::new(),
            value: "".into(),
            fetching: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {

        match msg {
            Msg::GotInput(new_value) => {
                self.value = new_value;
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

                let mut uri = "http://127.0.0.1:8000/add/".to_string();
                uri.push_str(&self.value);
                let request = Request::post(uri.as_str()).body(Nothing).unwrap();
                let task = self.fetch_service.fetch(request,callback);
            }

            Msg::GetList => {
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

                let request = Request::get("http://127.0.0.1:8000/list").body(Nothing).unwrap();
                let task = self.fetch_service.fetch(request,callback);
            }

            Msg::Remove => {
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

                let mut uri = "http://127.0.0.1:8000/remove/".to_string();
                uri.push_str(&self.value);
                let request = Request::put(uri.as_str()).body(Nothing).unwrap();
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
                    <input
                        value=&self.value
                        oninput=self.link.callback(|e: InputData| Msg::GotInput(e.value))
                        placeholder="Type item to add or remove">
                    </input>
                    <div>
                        <button onclick=self.link.callback(|_| Msg::Add)>{ "Add Item" }</button>
                        <button onclick=self.link.callback(|_| Msg::Remove)>{ "Remove Item" }</button>
                        <button onclick=self.link.callback(|_| Msg::GetList)>{ "Get List" }</button>
                    </div>
                </div>
                <div>
                    {&self.value}
                </div>
            </div>
        }
    }
}