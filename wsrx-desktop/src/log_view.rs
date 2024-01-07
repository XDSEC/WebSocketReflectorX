use adw::prelude::*;
use relm4::*;
use relm4_icons::icon_name;

pub struct LogViewModel;

#[relm4::component(pub)]
impl SimpleComponent for LogViewModel {
  type Init = ();
  type Input = ();
  type Output = ();

  view! {
    #[root]
    gtk::Box {
      gtk::Label {
        set_label: "Log",
      }
    }
  }

  fn init(
          init: Self::Init,
          root: &Self::Root,
          sender: ComponentSender<Self>,
      ) -> ComponentParts<Self> {
        let model = LogViewModel;
        let widgets = view_output!();
        ComponentParts { model, widgets }
  }
}
