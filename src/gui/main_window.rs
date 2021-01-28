/*
use iced::{button, Align, Button, Column, Element, Sandbox, Settings, Text};


#[derive(Default)]
pub struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
    refresh_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    RefreshPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::RefreshPressed => {
                self.refresh_button.is_disabled = false;
                let version_list_response = reqwest::blocking::get(crate::launcher_config::URL_JSON_VERSION_LIST_INOKI)
                    .unwrap().json::<crate::download::version_list::MinecraftVersionListJson>().unwrap();
                self.refresh_button.is_disabled = true;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .push(
                Button::new(&mut self.refresh_button, Text::new("Refresh"))
                    .on_press(Message::RefreshPressed)
            )
            .into()
    }
}
*/
use orbtk::prelude::*;

#[derive(Default, AsAny)]
pub struct MainState {
}

impl State for MainState {
    fn update(&mut self, _: &mut Registry, _: &mut Context) {
    }
}

widget!(
    MainView<MainState> {
        title: String16
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(TextBlock::new().text(("title", id)).build(ctx))
    }
}

pub fn launch_main_window() {
    // Counter::run(Settings::default());
    Application::from_name("{{project-name}}")
        .window(move |ctx| {
            Window::new()
                .title("{{project-name}}")
                .position((100.0, 100.0))
                .size(372.0, 768.0)
                .resizeable(true)
                .child(MainView::new().title("Hello OrbTk").build(ctx))
                .build(ctx)
        })
        .run();
}
