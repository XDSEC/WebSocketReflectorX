use adw::prelude::*;
use relm4::*;
use relm4_icons::icon_name;

pub struct StartViewModel;

#[relm4::component(pub)]
impl SimpleComponent for StartViewModel {
  type Init = ();
  type Input = ();
  type Output = ();

  view! {
    #[root]
    gtk::Box {
      gtk::Label {
        set_label: "Start",
      }
    }
  }

  fn init(
          init: Self::Init,
          root: &Self::Root,
          sender: ComponentSender<Self>,
      ) -> ComponentParts<Self> {
        let model = StartViewModel;
        let widgets = view_output!();
        ComponentParts { model, widgets }
  }
}
