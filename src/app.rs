use cosmic::app::{Command, Core};
use cosmic::iced::wayland::popup::{destroy_popup, get_popup};
use cosmic::iced::window::Id;
use cosmic::iced::Limits;
use cosmic::iced_style::application;
use cosmic::iced_widget::row;
use cosmic::widget::{self};
use cosmic::{Application, Element, Theme};

use crate::fl;

use crate::disk::{get_home_partition, get_partition, Partition};

#[derive(Default)]
pub struct App {
    core: Core,
    popup: Option<Id>,
    partitions: Vec<Partition>,
}

#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
}

impl Application for App {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "another.lusitano.AppletDisk";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = App {
            core,
            partitions: get_partition(),
            ..Default::default()
        };

        (app, Command::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<Self::Message> {
        let partitions = &self.partitions;
        let home_partition = get_home_partition(partitions).unwrap();
        let space_percentage = home_partition.get_space_percentage();

        let percentage = format!("{}%", space_percentage);

        row![
            self.core.applet.text(percentage),
            self.core
                .applet
                .icon_button("disks-symbolic")
                .on_press(Message::TogglePopup)
        ]
        .align_items(cosmic::iced::Alignment::Center)
        .into()
    }

    fn view_window(&self, _id: Id) -> Element<Self::Message> {
        let mut grid = widget::grid()
            .push(widget::text(fl!("partition")))
            .push(cosmic::iced::widget::vertical_rule(4))
            .push(widget::text(fl!("total-space")))
            .push(cosmic::iced::widget::vertical_rule(4))
            .push(widget::text(fl!("available-space")))
            .row_spacing(5)
            .padding(5.into());

        for partition in &self.partitions {
            let partition_name = &partition.mount_point;

            let total_disk_space = format!("{}GB", partition.get_total_space_in_gb());
            let available_disk_space = format!("{}GB", partition.get_available_space_in_gb());

            grid = grid
                .insert_row()
                .push(cosmic::iced::widget::horizontal_rule(4))
                .push(cosmic::iced::widget::vertical_rule(4))
                .push(cosmic::iced::widget::horizontal_rule(4))
                .push(cosmic::iced::widget::vertical_rule(4))
                .push(cosmic::iced::widget::horizontal_rule(4))
                .row_spacing(5)
                .insert_row()
                .push(widget::text(partition_name))
                .push(cosmic::iced::widget::vertical_rule(4))
                .push(widget::text(total_disk_space))
                .push(cosmic::iced::widget::vertical_rule(4))
                .push(widget::text(available_disk_space.to_string()))
                .row_spacing(5)
                .column_alignment(cosmic::iced::Alignment::Center)
        }

        self.core.applet.popup_container(grid).into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings =
                        self.core
                            .applet
                            .get_popup_settings(Id::MAIN, new_id, None, None, None);
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(400.0)
                        .min_width(100.0)
                        .min_height(100.0)
                        .max_height(200.0);
                    get_popup(popup_settings)
                };
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
        }
        Command::none()
    }

    fn style(&self) -> Option<<Theme as application::StyleSheet>::Style> {
        Some(cosmic::applet::style())
    }
}
