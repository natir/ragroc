use yew::prelude::*;

pub struct Landing {}

impl Component for Landing {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "Ragroc" }</h1>
            <h2>{ "Random dnA Generator and Reverse tnemelpmOC" }</h2>
            <button>{ "Random" }</button>
            <button>{ "Revcomp" }</button>
            </>
        }
    }
}
