use yew::prelude::*;

pub struct Navigation {}

impl Component for Navigation {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav>
            <div class="menu">
        <button>{ "Ragroc" }</button>
        <button>{ "Random" }</button>
        <button>{ "Revcomp" }</button>
            </div>
            </nav>
        }
    }
}
