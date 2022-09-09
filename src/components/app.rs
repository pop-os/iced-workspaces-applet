use std::{cmp::Ordering, env};

use adw_user_colors_lib::notify::*;
use calloop::channel::SyncSender;
use cosmic_panel_config::PanelAnchor;
use cosmic_protocols::workspace::v1::client::zcosmic_workspace_handle_v1;
use iced::theme::palette::Extended;
use iced::theme::{Palette, self};
use iced::widget::{button, column, container, row, text};
use iced::{
    executor, window, Alignment, Application, Command, Element, Length, Settings, Subscription,
    Theme,
};

use crate::config;
use crate::wayland::{WorkspaceEvent, WorkspaceList};
use crate::wayland_subscription::{workspaces, WorkspacesUpdate};

pub fn run() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.decorations = false;
    IcedWorkspacesApplet::run(settings)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Row,
    Column,
}

struct IcedWorkspacesApplet {
    theme: Theme,
    workspaces: WorkspaceList,
    workspace_tx: Option<SyncSender<WorkspaceEvent>>,
    layout: Layout,
}

#[derive(Debug, Clone)]
enum Message {
    PaletteChanged(Palette),
    WorkspaceUpdate(WorkspacesUpdate),
    WorkspacePressed(String),
}

impl Application for IcedWorkspacesApplet {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            IcedWorkspacesApplet {
                layout: match env::var("COSMIC_PANEL_ANCHOR")
                    .ok()
                    .and_then(|anchor| anchor.parse::<PanelAnchor>().ok())
                    .unwrap_or_default()
                {
                    PanelAnchor::Left | PanelAnchor::Right => Layout::Column,
                    PanelAnchor::Top | PanelAnchor::Bottom => Layout::Row,
                },
                theme: Default::default(),
                workspaces: Default::default(),
                workspace_tx: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        config::APP_ID.to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PaletteChanged(palette) => {
                self.theme = Theme::Custom {
                    palette,
                    extended: Extended::generate(palette),
                }
            }
            Message::WorkspaceUpdate(msg) => match msg {
                WorkspacesUpdate::Workspaces(mut list) => {
                    list.sort_by(|a, b| match a.0.len().cmp(&b.0.len()) {
                        Ordering::Equal => a.0.cmp(&b.0),
                        Ordering::Less => Ordering::Less,
                        Ordering::Greater => Ordering::Greater,
                    });
                    self.workspaces = list;
                    let unit = 24;
                    let (w, h) = match self.layout {
                        Layout::Row => (unit * self.workspaces.len() as u32, unit),
                        Layout::Column => (unit, unit * self.workspaces.len() as u32),
                    };
                    return window::resize(w, h);
                }
                WorkspacesUpdate::Started(tx) => {
                    self.workspace_tx.replace(tx);
                }
                WorkspacesUpdate::Errored => {
                    // TODO
                }
            },
            Message::WorkspacePressed(name) => if let Some(tx) = self.workspace_tx.as_mut() {
                let _ = tx.try_send(WorkspaceEvent::Activate(name));
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let buttons = self
            .workspaces
            .iter()
            .filter_map(|w| {
                let btn = button(text(w.0.clone()))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .on_press(Message::WorkspacePressed(w.0.clone()));
                match w.1 {
                    Some(zcosmic_workspace_handle_v1::State::Active) => Some(btn.into()),
                    Some(zcosmic_workspace_handle_v1::State::Urgent) => Some(btn.style(theme::Button::Destructive).into()),
                    None => Some(btn.style(theme::Button::Secondary).into()),
                    _ => None,
                }
            })
            .collect();
        let layout_section: Element<_> = match self.layout {
            Layout::Row => row(buttons).width(Length::Fill).height(Length::Fill).into(),
            Layout::Column => column(buttons)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        };

        container(layout_section)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(
            vec![
                theme(0).map(|(_, theme_update)| match theme_update {
                    ThemeUpdate::Palette(palette) => Message::PaletteChanged(palette),
                    ThemeUpdate::Errored => todo!(),
                }),
                workspaces(0).map(|(_, msg)| Message::WorkspaceUpdate(msg)),
            ]
            .into_iter(),
        )
    }

    fn theme(&self) -> Theme {
        self.theme
    }
}
