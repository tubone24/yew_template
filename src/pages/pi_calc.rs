use crate::logics::pi_calc::gauss_legendre;
use std::str::FromStr;
use std::fmt::{Debug, Formatter};
use yew::prelude::*;
use yew_styles::forms::form_input::{
    FormInput,
    InputType,
};
use yew_styles::styles::Size;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

use bigdecimal::BigDecimal;

#[derive(Debug)]
pub enum Msg {
    Calc,
    InputDigit(String),
}

pub struct PiCalc {
    link: ComponentLink<Self>,
    state: State
}

struct State {
    digit: String,
    result: BigDecimal,
}

impl Component for PiCalc {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            digit: "10".to_string(),
            result: BigDecimal::from_str("0").unwrap(),
        };
        PiCalc {
            link,
            state
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("Update: {:?}", msg);
        match msg {
            Msg::InputDigit(value) => {
                log::info!("value: {:?}",value);
                self.state.digit = value;
            }
            Msg::Calc => {
                log::info!("calc!!!");
                let n: i64 = self.state.digit.parse().unwrap();
                let t: BigDecimal = gauss_legendre(n);
                log::info!("result: {:?}", format!("{}", t));
                self.state.result = t;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap class_name="content">
                <Item layouts=vec!(ItemLayout::ItM(12))>
                    <h2>{"Calc PI by wasm!"}</h2>
                </Item>
                <Item layouts=vec!(ItemLayout::ItM(12))>
                    {"precision: "} <input type="text" value=self.state.digit.clone()  oninput=self.link.callback(|e:InputData| Msg::InputDigit(e.value))/>
                    <button onclick=self.link.callback(|_| Msg::Calc)>{ "Calc" }</button>
                </Item>
                <Item layouts=vec!(ItemLayout::ItM(12))>
                    <p>{self.state.result.to_string()}</p>
                </Item>
            </Container>
        }
    }
}
