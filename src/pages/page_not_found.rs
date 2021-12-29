use yew::prelude::*;
use yew_styles::{
    text::{
        Text,
        TextType,
    },
    styles::{
        Size,
    },
};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};

#[derive(Properties, Clone)]
pub struct Props {
    pub missed_route: std::rc::Rc<String>
}

pub struct PageNotFound {
    props: Props
}

impl Component for PageNotFound {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
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
                    <Text
                        text_type=TextType::Plain
                        text_size=Size::Big
                        html_text=Some(html!{<>{"Page "}<i><b>{self.props.missed_route.clone()}</b></i>{" is not found..."}</>})
                    />
                </Item>
            </Container>
        }
    }
}
