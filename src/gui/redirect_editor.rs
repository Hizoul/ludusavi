use crate::{
    config::Config,
    gui::{
        common::{BrowseSubject, EditAction, Message, RedirectEditActionField},
        icon::Icon,
        style,
    },
    lang::Translator,
    shortcuts::TextHistory,
};

use iced::{button, scrollable, text_input, Button, Container, Length, Row, Scrollable, Space, TextInput};

#[derive(Default)]
pub struct RedirectEditorRow {
    button_state: button::State,
    pub source_text_state: text_input::State,
    pub source_text_history: TextHistory,
    source_browse_button_state: button::State,
    pub target_text_state: text_input::State,
    pub target_text_history: TextHistory,
    target_browse_button_state: button::State,
}

impl RedirectEditorRow {
    pub fn new(initial_source: &str, initial_target: &str) -> Self {
        Self {
            source_text_history: TextHistory::new(initial_source, 100),
            target_text_history: TextHistory::new(initial_target, 100),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct RedirectEditor {
    scroll: scrollable::State,
    pub rows: Vec<RedirectEditorRow>,
}

impl RedirectEditor {
    pub fn view(&mut self, config: &Config, translator: &Translator) -> Container<Message> {
        let redirects = config.get_redirects();
        if redirects.is_empty() {
            return Container::new(Space::new(Length::Shrink, Length::Shrink));
        }

        Container::new({
            self.rows.iter_mut().enumerate().fold(
                Scrollable::new(&mut self.scroll)
                    .width(Length::Fill)
                    // TODO: https://github.com/iced-rs/iced/issues/1388
                    .height(if config.restore.redirects.len() > 3 {
                        Length::Units(100)
                    } else {
                        Length::Shrink
                    })
                    .max_height(100)
                    .spacing(5)
                    .style(style::Scrollable(config.theme)),
                |parent: Scrollable<'_, Message>, (i, x)| {
                    parent.push(
                        Row::new()
                            .padding([0, 20, 0, 20])
                            .spacing(20)
                            .push(
                                Button::new(&mut x.button_state, Icon::RemoveCircle.as_text())
                                    .on_press(Message::EditedRedirect(EditAction::Remove(i), None))
                                    .style(style::Button::Negative(config.theme)),
                            )
                            .push(
                                TextInput::new(
                                    &mut x.source_text_state,
                                    &translator.redirect_source_placeholder(),
                                    &redirects[i].source.raw(),
                                    move |v| {
                                        Message::EditedRedirect(
                                            EditAction::Change(i, v),
                                            Some(RedirectEditActionField::Source),
                                        )
                                    },
                                )
                                .style(style::TextInput(config.theme))
                                .width(Length::FillPortion(3))
                                .padding(5),
                            )
                            .push(
                                Button::new(&mut x.source_browse_button_state, Icon::FolderOpen.as_text())
                                    .on_press(Message::BrowseDir(BrowseSubject::RedirectSource(i)))
                                    .style(style::Button::Primary(config.theme)),
                            )
                            .push(
                                TextInput::new(
                                    &mut x.target_text_state,
                                    &translator.redirect_target_placeholder(),
                                    &redirects[i].target.raw(),
                                    move |v| {
                                        Message::EditedRedirect(
                                            EditAction::Change(i, v),
                                            Some(RedirectEditActionField::Target),
                                        )
                                    },
                                )
                                .style(style::TextInput(config.theme))
                                .width(Length::FillPortion(3))
                                .padding(5),
                            )
                            .push(
                                Button::new(&mut x.target_browse_button_state, Icon::FolderOpen.as_text())
                                    .on_press(Message::BrowseDir(BrowseSubject::RedirectTarget(i)))
                                    .style(style::Button::Primary(config.theme)),
                            ),
                    )
                },
            )
        })
    }
}
