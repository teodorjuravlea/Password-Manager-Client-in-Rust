use std::{cell::RefCell, rc::Rc};

use crate::AppState;

use super::auth_prompt::AuthPrompt;
use super::main_window::MainWindow;
use relm4::{
    adw, component::Connector, gtk, Component, ComponentController, ComponentParts,
    ComponentSender, Controller, SimpleComponent,
};

use adw::{prelude::*, ApplicationWindow};
use gtk::{Box, Stack, StackTransitionType};

pub struct AppModel {
    page: Page,
    auth: Connector<AuthPrompt>,
    main: Controller<MainWindow>,
}

enum Page {
    Auth,
    Main,
}

#[derive(Debug)]
pub enum AppMessage {
    LoginSuccessful,
}

#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Widgets = AppWidgets;
    type Init = Rc<RefCell<AppState>>;
    type Input = AppMessage;
    type Output = ();

    view! {
        window = ApplicationWindow {
            // add_css_class: "devel",
            set_default_size: (1024, 600),

            #[name = "stack"]
            Stack {
                set_transition_type: StackTransitionType::SlideLeft,
                add_child: auth_page = &Box {
                    append: model.auth.widget(),
                },
                add_child: main_page = &Box {
                    append: model.main.widget(),
                },
            }
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMessage::LoginSuccessful => self.page = Page::Main,
        }
    }

    fn pre_view() {
        match model.page {
            Page::Auth => stack.set_visible_child(auth_page),
            Page::Main => stack.set_visible_child(main_page),
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            page: Page::Auth,
            auth: AuthPrompt::builder().launch(init.clone()),
            main: MainWindow::builder().launch(init.clone()).detach(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
