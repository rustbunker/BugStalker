use crate::debugger::command;
use crate::ui::tui::app::port::UserEvent;
use crate::ui::tui::proto::ClientExchanger;
use crate::ui::tui::{Id, Msg};
use nix::sys::signal::Signal;
use std::sync::Arc;
use tuirealm::event::{Key, KeyEvent, KeyModifiers};
use tuirealm::{Component, Event, MockComponent, Sub, SubClause, SubEventClause};

#[derive(MockComponent)]
pub struct GlobalControl {
    component: tui_realm_stdlib::Phantom,
    exchanger: Arc<ClientExchanger>,
}

impl GlobalControl {
    pub fn new(exchanger: Arc<ClientExchanger>) -> Self {
        Self {
            component: tui_realm_stdlib::Phantom::default(),
            exchanger,
        }
    }

    pub fn subscriptions() -> Vec<Sub<Id, UserEvent>> {
        vec![
            Sub::new(
                SubEventClause::Keyboard(KeyEvent::new(Key::Char('1'), KeyModifiers::NONE)),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent::new(Key::Char('2'), KeyModifiers::NONE)),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent::new(Key::Char('c'), KeyModifiers::ALT)),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent::new(Key::Esc, KeyModifiers::NONE)),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent::new(Key::Char('q'), KeyModifiers::NONE)),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent::new(Key::Char('c'), KeyModifiers::NONE)),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent::new(Key::Char('c'), KeyModifiers::CONTROL)),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent::new(Key::Char('r'), KeyModifiers::NONE)),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent::new(Key::Function(6), KeyModifiers::NONE)),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent::new(Key::Function(7), KeyModifiers::NONE)),
                SubClause::Always,
            ),
            Sub::new(
                SubEventClause::Keyboard(KeyEvent::new(Key::Function(8), KeyModifiers::NONE)),
                SubClause::Always,
            ),
            Sub::new(
                // concrete signal doesn't meter
                SubEventClause::User(UserEvent::Signal(Signal::SIGUSR2)),
                SubClause::Always,
            ),
            Sub::new(
                // concrete error doesn't meter
                SubEventClause::User(UserEvent::AsyncErrorResponse(String::default())),
                SubClause::Always,
            ),
        ]
    }
}

impl Component<Msg, UserEvent> for GlobalControl {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        let msg = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char('1'),
                modifiers: KeyModifiers::NONE,
            }) => Msg::LeftTabsInFocus,
            Event::Keyboard(KeyEvent {
                code: Key::Char('2'),
                modifiers: KeyModifiers::NONE,
            }) => Msg::RightTabsInFocus,
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::ALT,
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => Msg::SwitchUI,
            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                modifiers: KeyModifiers::NONE,
            })
            | Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => Msg::AppClose,
            Event::Keyboard(KeyEvent {
                code: Key::Char('c'),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.exchanger
                    .request_async(|dbg| Ok(command::Continue::new(dbg).handle()?));
                Msg::AppRunning
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('r'),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.exchanger
                    .request_async(|dbg| Ok(command::Run::new(dbg).start()?));
                Msg::AppRunning
            }
            Event::User(UserEvent::Signal(sig)) => {
                Msg::ShowAlert(format!("Application receive signal: {sig}"))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Function(8),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.exchanger
                    .request_async(|dbg| Ok(command::StepOver::new(dbg).handle()?));
                Msg::AppRunning
            }
            Event::Keyboard(KeyEvent {
                code: Key::Function(7),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.exchanger
                    .request_async(|dbg| Ok(command::StepInto::new(dbg).handle()?));
                Msg::AppRunning
            }
            Event::Keyboard(KeyEvent {
                code: Key::Function(6),
                modifiers: KeyModifiers::NONE,
            }) => {
                self.exchanger
                    .request_async(|dbg| Ok(command::StepOut::new(dbg).handle()?));
                Msg::AppRunning
            }
            Event::User(UserEvent::AsyncErrorResponse(err)) => Msg::ShowAlert(err),
            _ => Msg::None,
        };
        Some(msg)
    }
}
