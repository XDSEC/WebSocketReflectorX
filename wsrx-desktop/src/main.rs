use about_view::AboutViewModel;
use adw::prelude::*;
use connection_view::ConnectionViewModel;
use log_view::LogViewModel;
use relm4::{ComponentParts, ComponentSender, RelmApp, SimpleComponent, Controller, Component, ComponentController};
use relm4_icons::icon_name;
use side_bar::{Page, SideBarModel, SideBarInput};
use start_view::StartViewModel;

use crate::side_bar::SideBarOutput;

mod about_view;
mod connection_view;
mod log_view;
mod side_bar;
mod start_view;

const APP_ID: &str = "tech.woooo.wsrx";

struct AppModel {
    page: Page,
    side_bar: Controller<SideBarModel>,
    start_view: Controller<StartViewModel>,
    connection_view: Controller<ConnectionViewModel>,
    log_view: Controller<LogViewModel>,
    about_view: Controller<AboutViewModel>,
}

#[derive(Debug)]
enum AppInput {
    SelectPage(Page),
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppInput;
    type Output = ();

    view! {
        #[root]
        adw::ApplicationWindow {
            set_title: Some("WebSocket Reflector X"),
            set_default_width: 1200,
            set_default_height: 700,
            set_width_request: 500,
            set_height_request: 400,

            add_breakpoint = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
                adw::BreakpointConditionLengthType::MaxWidth,
                600.0,
                adw::LengthUnit::Sp,
            )) {
                add_setter: (
                    &main_view,
                    "collapsed",
                    &true.into(),
                ),
            },

            #[name(main_view)]
            adw::OverlaySplitView {
                set_max_sidebar_width: 320.0,
                set_min_sidebar_width: 200.0,
                set_sidebar_width_fraction: 0.32,
                #[wrap(Some)]
                set_sidebar = &adw::NavigationPage {
                    set_title: "WEBSOCKET REFLECTOR X",
                    set_can_pop: true,
                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {},
                        #[wrap(Some)]
                        set_content = model.side_bar.widget(),
                    }
                },
                #[wrap(Some)]
                set_content = &adw::NavigationPage {
                    set_title: "Page Stack",
                    #[wrap(Some)]
                    set_child = &adw::ToolbarView {
                        add_top_bar = &adw::HeaderBar {
                            set_show_title: false,
                            pack_start = &gtk::Button {
                                set_icon_name: icon_name::NAVIGATION_REGULAR,
                                connect_clicked[main_view] => move |_| {
                                    main_view.set_show_sidebar(true);
                                },
                            },
                        },

                        #[wrap(Some)]
                        #[name(stack)]
                        set_content = &gtk::Stack {
                            set_transition_type: gtk::StackTransitionType::RotateLeft,
                            add_child: model.start_view.widget(),
                            add_child: model.connection_view.widget(),
                            add_child: model.log_view.widget(),
                            add_child: model.about_view.widget(),
                        }
                    }
                },
            }
       }
    }

    // Initialize the UI.
    fn init(
        _counter: Self::Init,
        _root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let side_bar: Controller<SideBarModel> = SideBarModel::builder().launch(Page::Start).forward(sender.input_sender(), |msg| {
            match msg {
                SideBarOutput::SelectPage(page) => AppInput::SelectPage(page),
            }
        });
        let start_view: Controller<StartViewModel> = StartViewModel::builder().launch(()).detach();
        let connection_view: Controller<ConnectionViewModel> = ConnectionViewModel::builder().launch(()).detach();
        let log_view: Controller<LogViewModel> = LogViewModel::builder().launch(()).detach();
        let about_view: Controller<AboutViewModel> = AboutViewModel::builder().launch(()).detach();
        let model = AppModel {
            page: Page::Start,
            side_bar,
            start_view,
            connection_view,
            log_view,
            about_view,
        };

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppInput::SelectPage(page) => {
                self.page = page.clone();
                self.side_bar.emit(SideBarInput::SelectPage(page));
            }
        }
    }
}

fn main() {
    let app = RelmApp::new(APP_ID);
    relm4_icons::initialize_icons();
    app.run::<AppModel>(());
}
