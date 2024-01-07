use adw::prelude::*;
use relm4::*;
use relm4_icons::icon_name;

pub struct SideBarModel {
    page: Page,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Page {
    Start,
    Connection,
    Log,
    About,
}

#[derive(Debug)]
pub enum SideBarInput {
    SelectPage(Page),
}

#[derive(Debug)]
pub enum SideBarOutput {
    SelectPage(Page),
}

#[relm4::component(pub)]
impl SimpleComponent for SideBarModel {
    type Init = Page;
    type Input = SideBarInput;
    type Output = SideBarOutput;

    view! {
        #[root]
        gtk::Box {
            set_spacing: 6,
            set_margin_all: 8,
            set_orientation: gtk::Orientation::Vertical,

            #[name = "group"]
            gtk::ToggleButton {
                #[watch]
                set_active: model.page == Page::Start,
                set_has_frame: false,

                gtk::Box {
                    set_spacing: 12,
                    set_margin_horizontal: 4,
                    set_orientation: gtk::Orientation::Horizontal,

                    gtk::Image {
                        set_from_icon_name: Some(icon_name::HOME_REGULAR),
                    },
                    gtk::Label {
                        set_label: "Get Started",
                    },
                },

                connect_toggled[sender] => move |btn| {
                    if btn.is_active() {
                        sender.output(SideBarOutput::SelectPage(Page::Start)).unwrap()
                    }
                },
            },
            gtk::ToggleButton {
                #[watch]
                set_active: model.page == Page::Connection,
                set_has_frame: false,
                set_group: Some(&group),

                gtk::Box {
                    set_spacing: 12,
                    set_margin_horizontal: 4,
                    set_orientation: gtk::Orientation::Horizontal,

                    gtk::Image {
                        set_from_icon_name: Some(icon_name::LINK_REGULAR),
                    },
                    gtk::Label {
                        set_label: "Connections",
                    },
                },

                connect_toggled[sender] => move |btn| {
                    if btn.is_active() {
                        sender.output(SideBarOutput::SelectPage(Page::Connection)).unwrap()
                    }
                },
            },

            gtk::ToggleButton {
                #[watch]
                set_active: model.page == Page::Log,
                set_has_frame: false,
                set_group: Some(&group),

                gtk::Box {
                    set_spacing: 12,
                    set_margin_horizontal: 4,
                    set_orientation: gtk::Orientation::Horizontal,

                    gtk::Image {
                        set_from_icon_name: Some(icon_name::DOCUMENT_REGULAR),
                    },
                    gtk::Label {
                        set_label: "Network Logs",
                    },
                },

                connect_toggled[sender] => move |btn| {
                    if btn.is_active() {
                        sender.output(SideBarOutput::SelectPage(Page::Log)).unwrap()
                    }
                },
            },

            gtk::Box {
                set_vexpand: true,
            },

            gtk::ToggleButton {
                #[watch]
                set_active: model.page == Page::About,
                set_has_frame: false,
                set_group: Some(&group),

                gtk::Box {
                    set_spacing: 12,
                    set_margin_horizontal: 4,
                    set_orientation: gtk::Orientation::Horizontal,

                    gtk::Image {
                        set_from_icon_name: Some(icon_name::INFO_REGULAR),
                    },
                    gtk::Label {
                        set_label: "About Program",
                    },
                },

                connect_toggled[sender] => move |btn| {
                    if btn.is_active() {
                        sender.output(SideBarOutput::SelectPage(Page::About)).unwrap()
                    }
                },
            },
        }
    }

    fn init(
        params: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SideBarModel { page: params };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            SideBarInput::SelectPage(p) => self.page = p,
        }
    }
}
