use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap class_name="content">
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h2>{"Yew Practice"}</h2>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <h3>{"Perform difficult calculations using Wasm."}</h3>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <ul>
                        <li><a href="/picalc">{"PI Calc"}</a>{" : Calculate π using the gauss legendre method."}</li>
                    </ul>
                </Item>
            </Container>
        }
    }
}
