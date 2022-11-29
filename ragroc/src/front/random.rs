use yew::prelude::*;

pub struct Random {}

impl Component for Random {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
        <h2>{ "Random" }</h2>
        <section>
        <textarea readonly=true></textarea>
        </section>
        <menu>
        <form id="random_param">
        <label for="nb_sequences">{ "Number of sequences:" }</label>
          <input id="nb_sequences" type="number" value="1" min="0"/>

        <label for="length">{ "Length:" }</label>
          <input id="length" type="number" value="100" min="0"/>

          <fieldset>
        <legend>{ "Format:" }</legend>
            <label>
              <input type="radio" name="format" id="none" checked=true/>
        <span class="checkable">{ "none" }</span>
            </label>
            <label>
              <input type="radio" name="format" id="fasta"/>
        <span class="checkable">{ "fasta" }</span>
            </label>
            <label>
              <input type="radio" name="format" id="fastq"/>
        <span class="checkable">{ "fastq" }</span>
            </label>
          </fieldset>

          <fieldset>
        <legend>{ "Weight of each nucleotide:" }</legend>
        <label  for="a_weight">{ "A:" }</label>
            <input id="a_weight"  type="number" value="0.25" max="1" min="0" step="0.01"/>
        <label  for="c_weight">{ "C:" }</label>
            <input id="c_weight"  type="number" value="0.25" max="1" min="0" step="0.01"/>
        <label  for="g_weight">{ "G:" }</label>
            <input id="g_weight"  type="number" value="0.25" max="1" min="0" step="0.01"/>
        <label  for="t_weight">{ "T:" }</label>
            <input id="t_weight"  type="number" value="0.25" max="1" min="0" step="0.01"/>
          </fieldset>

          <div class="large-button">
            <input id="generate_submit" class="warning" type="submit" value="Submit"/>
          </div>

          <fieldset>
            <input id="gc_percent" for="gc_percent_input" class="button" type="button" value="Set gc%"/>
            <input id="gc_percent_input"  type="number" value="0.5" min="0" max="1" step="0.01"/>
          </fieldset>
        </form>
        </menu>
            </>
        }
    }
}
