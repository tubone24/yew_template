use std::ffi::c_void;
use crate::logics::pi_calc::gauss_legendre;
use std::str::FromStr;
use std::fmt::{Debug};
use yew::prelude::*;
use yew_styles::text::{
    TextType,
    Text,
};
use stylist::css;
use yew_styles::styles::{
    Size,
    Palette,
};
use yew_styles::spinner::{
    Spinner,
    SpinnerType
};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

use bigdecimal::{BigDecimal, ToPrimitive};

#[derive(Debug)]
pub enum Msg {
    Calc,
    InputDigit(String),
    Calculating,
    Calculated,
}

pub struct PiCalc {
    link: ComponentLink<Self>,
    state: State
}

struct State {
    digit: String,
    calculating: bool,
    result: BigDecimal,
}

impl Component for PiCalc {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            digit: "10".to_string(),
            calculating: false,
            result: BigDecimal::from_str("0").unwrap(),
        };
        PiCalc {
            link,
            state
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("Update: {:?}", msg);
        let cb1 = self.link.callback(|_: Option<String>| Msg::Calculating);
        let cb2 = self.link.callback(|_: Option<String>| Msg::Calculated);
        match msg {
            Msg::InputDigit(value) => {
                self.state.digit = value;
            }
            Msg::Calc => {
                cb1.emit(None);
                let n: i64 = self.state.digit.parse().unwrap();
                let t: BigDecimal = gauss_legendre(n);
                log::info!("result: {:?}", t);
                self.state.result = t;
                cb2.emit(None);
            }
            Msg::Calculating => {
                self.state.calculating = true;
            }
            Msg::Calculated => {
                self.state.calculating = false;
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
            {
                if self.state.calculating {
                    html! {
                        <Spinner
                            spinner_type=SpinnerType::Circle
                            spinner_size=Size::Medium
                            spinner_palette=Palette::Info/>
                    }
                } else {
                    html! {
                        <></>
                    }
                }
            }
                <Item layouts=vec!(ItemLayout::ItM(12))>
                    <h2>{"Calc PI by wasm!"}</h2>
                </Item>
                <Item layouts=vec!(ItemLayout::ItM(12))>
                    {"precision: "} <input type="text" value=self.state.digit.clone()  oninput=self.link.callback(|e:InputData| Msg::InputDigit(e.value))/>
                    <button onclick=self.link.callback(|_| Msg::Calc)>{ "Calc" }</button>
                </Item>
                <Item layouts=vec!(ItemLayout::ItM(12))>
                    <Text
                        text_type=TextType::Paragraph
                        text_size=Size::Small
                        plain_text={self.state.result.to_string()}
                        html_text=None
                        styles=css!("
                            overflow-wrap: break-word;
                            word-break: break-all;
                        ")
                    />
                </Item>
            </Container>
        }
    }
}
