use cooltraption_window::events::EventHandler;
use cooltraption_window::window::{
    winit::{
        self,
        event::{ElementState, KeyboardInput, VirtualKeyCode},
    },
    WindowContext, WinitEvent,
};

use cooltraption_common::events::EventPublisher;

#[derive(Default)]
struct InputEventHandler<'a> {
    event_publisher: EventPublisher<'a, InputEvent>,
}

pub enum InputEvent {
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
}

impl<'a> InputEventHandler<'a> {
    pub fn new(event_publisher: EventPublisher<'a, InputEvent>) -> Self {
        Self { event_publisher }
    }

    fn keyboard_input(&mut self, input: &mut KeyboardInput) {
        if let Some(key_code) = input.virtual_keycode {
            let event = match input.state {
                ElementState::Pressed => InputEvent::KeyPressed(key_code),
                ElementState::Released => InputEvent::KeyReleased(key_code),
            };
            self.event_publisher.publish(&event);
        }
    }
}

impl<'a> EventHandler<WinitEvent<'_, '_>, WindowContext<'_>> for InputEventHandler<'a> {
    fn handle_event(&mut self, event: &mut WinitEvent, _context: &mut WindowContext) {
        if let winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::KeyboardInput { input, .. },
            ..
        } = event.0
        {
            self.keyboard_input(input)
        }
    }
}
