use yew::prelude::*;

pub struct RevComp {}

impl Component for RevComp {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
        <h2>{ "RevComp" }</h2>
        <section>
        <textarea id="rc_in_text"></textarea>
        </section>
        <section>
        <button id="rc_submit">{ "Submit" }</button>
        </section>
        <section>
        <textarea id="rc_out_text" readonly=true></textarea>
        </section>
        </>
        }
    }
}
