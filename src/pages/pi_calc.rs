use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use num_bigint::BigUint;

pub struct PiCalc;

struct State {
    digit: i32,
    result: BigUint,
}

impl Component for PiCalc {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        PiCalc {}
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
                    <h2>{"Thanks for using or contributing!"}</h2>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12))>
                    <p>{"Yew Parcel Template is a "}
                        <a href="https://github.com/spielrs/yew-parcel-template/blob/master/LICENSE">{"MIT licensed "}</a>
                        {"project maintained by open source comunity"}
                    </p>
                </Item>
            </Container>
        }
    }
}
