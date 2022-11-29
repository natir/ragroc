use yew::prelude::*;

pub struct App {
    title: String,
    msg: String,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            title: "xtask-wasm".to_string(),
            msg: "Hello new world".to_string(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{self.title.clone()}</h1>
                <p>{self.msg.clone()}</p>
            </div>
        }
    }
}
