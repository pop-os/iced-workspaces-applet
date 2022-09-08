use std::cmp::Ordering;

use adw_user_colors_lib::notify::*;
use calloop::channel::SyncSender;
use cosmic_panel_config::PanelAnchor;
use cosmic_protocols::workspace::v1::client::zcosmic_workspace_handle_v1;
use iced::theme::palette::Extended;
use iced::theme::Palette;
use iced::widget::{
    column, container, text
};
use iced::{Alignment, Element, Length, Settings, Theme, Application, executor, Command, Subscription};

use crate::config;
use crate::wayland::{WorkspaceList, WorkspaceEvent};
use crate::wayland_subscription::{workspaces, WorkspacesUpdate};

pub fn run() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.decorations = false;
    IcedWorkspacesApplet::run(settings)
}

#[derive(Default)]
struct IcedWorkspacesApplet {
    theme: Theme,
    workspaces: WorkspaceList,
    workspace_tx: Option<SyncSender<WorkspaceEvent>>,
    anchor: PanelAnchor,
}

#[derive(Debug, Clone)]
enum Message {
    PaletteChanged(Palette),
    WorkspaceUpdate(WorkspacesUpdate),
}

impl Application for IcedWorkspacesApplet {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();
    
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            IcedWorkspacesApplet {
                anchor: std::env::var("COSMIC_PANEL_ANCHOR")
                    .ok()
                    .and_then(|anchor| anchor.parse::<PanelAnchor>().ok())
                    .unwrap_or_default(),
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        config::APP_ID.to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message>{
        match message {
            Message::PaletteChanged(palette) => {self.theme = Theme::Custom {
                palette,
                extended: Extended::generate(palette),
            }},
            Message::WorkspaceUpdate(msg) => match msg {
                WorkspacesUpdate::Workspaces(mut list) => {
                    list.sort_by(|a, b| {
                        match a.0.len().cmp(&b.0.len()) {
                            Ordering::Equal => a.0.cmp(&b.0),
                            Ordering::Less => Ordering::Less,
                            Ordering::Greater => Ordering::Greater,
                        }
                    });
                    self.workspaces = list;
                },
                WorkspacesUpdate::Started(tx) => {self.workspace_tx.replace(tx);},
                WorkspacesUpdate::Errored => {
                    // TODO
                },
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            text("hello world")
            .height(Length::Units(100))
        ]
        .spacing(20)
        .padding(20)
        .max_width(600);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            theme(0).map(|(_, theme_update)| match theme_update {
                ThemeUpdate::Palette(palette) => Message::PaletteChanged(palette),
                ThemeUpdate::Errored => todo!(),
            }),
            workspaces(0).map(|(_, msg)| Message::WorkspaceUpdate(msg))
        ].into_iter())
    }

    fn theme(&self) -> Theme {
        self.theme
    }
}
