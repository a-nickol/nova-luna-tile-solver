use crate::model::nova_luna::State;
use yew::prelude::*;

#[derive(Debug)]
pub struct GameBoard {
    state: State,
}

impl Component for GameBoard {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            state: State::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {

            <main>
                <h1 class="block mx-auto h-24 rounded-full">{ "Nova Luna Tile Solver!" }</h1>
            </main>
        }
    }
}
