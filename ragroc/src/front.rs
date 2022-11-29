use yew::prelude::*;

mod landing;
mod navigation;
mod random;
mod revcomp;

use landing::*;
use navigation::*;
use random::*;
use revcomp::*;

pub struct Ragroc {}

impl Component for Ragroc {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <>
            <Navigation/>
        <main>
        <Landing/>
        <Random/>
        <RevComp/>
        </main>
        </>
        }
    }
}
