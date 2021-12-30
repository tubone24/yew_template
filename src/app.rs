use crate::pages::{About, Home, PageNotFound, PiCalc};
use std::rc::Rc;
use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};
use yew_styles::{
    navbar::{
        navbar_component::{Fixed, Navbar},
        navbar_container::NavbarContainer,
        navbar_item::NavbarItem,
    },
    styles::{Palette, Style},
};

pub struct App {
    navbar_items: Vec<bool>,
    link: ComponentLink<Self>,
}

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/!"]
    RootPath,
    #[to = "/about!"]
    AboutPath,
    #[to = "/picalc!"]
    PiCalcPath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

pub enum Msg {
    ChangeNavbarItem(usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            navbar_items: vec![true, false, false],
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeNavbarItem(index) => {
                for (i, _) in self.navbar_items.clone().into_iter().enumerate() {
                    self.navbar_items[i] = false;
                }

                self.navbar_items[index] = true;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Navbar
                    class_name="navbar-router"
                    navbar_palette=Palette::Standard
                    navbar_style=Style::Light
                    fixed=Fixed::Top
                    branch=html!{<img src="https://avatars.githubusercontent.com/u/9511227?v=4"/>}
                >
                    <NavbarContainer>
                        <NavbarItem
                            class_name="navbar-route"
                            active = self.navbar_items[0]
                            onclick_signal = self.link.callback(|_| Msg::ChangeNavbarItem(0))
                            >
                            <RouterAnchor<AppRouter>route=AppRouter::RootPath>{"Home"}</RouterAnchor<AppRouter>></NavbarItem>
                        <NavbarItem
                            class_name="navbar-route"
                            active = self.navbar_items[1]
                            onclick_signal = self.link.callback(|_| Msg::ChangeNavbarItem(1))
                            >
                            <RouterAnchor<AppRouter>route=AppRouter::AboutPath>{"About"}</RouterAnchor<AppRouter>></NavbarItem>
                        <NavbarItem
                            class_name="navbar-route"
                            active = self.navbar_items[2]
                            onclick_signal = self.link.callback(|_| Msg::ChangeNavbarItem(2))
                            >
                            <RouterAnchor<AppRouter>route=AppRouter::PiCalcPath>{"PiCalc"}</RouterAnchor<AppRouter>></NavbarItem>
                    </NavbarContainer>
                </Navbar>
                <Router<AppRouter, ()>
                    render = Router::render(|switch: AppRouter | {
                        match switch {
                            AppRouter::RootPath => html!{
                                <Home/>
                            },
                            AppRouter::AboutPath => html!{
                                <About/>
                            },
                            AppRouter::PiCalcPath => html!{
                                <PiCalc/>
                            },
                            AppRouter::PageNotFound(Permissive(None)) => html!{
                                <PageNotFound missed_route=Rc::new("".to_string()) />
                            },
                            AppRouter::PageNotFound(Permissive(Some(missed_route))) => html!{
                                <PageNotFound missed_route=Rc::new(missed_route) />
                            }
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRouter::PageNotFound(Permissive(Some(route.route)))
                    })
                />
            </div>
        }
    }
}
