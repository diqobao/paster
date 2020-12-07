use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    uuid: Option<String>,
    content_type: String,
    value: String,
    ttl: i32,
}

struct State {
    entry: Entry,
}

enum Msg {
    DeleteEntry,
    CreateEntry,
    UpdateEntry,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let entry = Entry {
            uuid: None,
            content_type: "",
            content: "",
        }
        let state = State {
            entry: entry,
        };

        Self {
            link,
            state: state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}