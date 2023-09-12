mod states;

use crate::engine::{Rect, Renderer};

use self::states::*;
use web_sys::HtmlImageElement;

use super::Sheet;

pub struct RedHatBoy {
    state_machine: RedHatBoyStateMachine,
    sprite_sheet: Sheet,
    image: HtmlImageElement,
}

#[derive(Copy, Clone)]
enum RedHatBoyStateMachine {
    Idle(RedHatBoyState<Idle>),
    Running(RedHatBoyState<Running>),
}

impl RedHatBoy {
    pub fn new(sprite_sheet: Sheet, image: HtmlImageElement) -> Self {
        RedHatBoy {
            state_machine: RedHatBoyStateMachine::Idle(RedHatBoyState::new()),
            sprite_sheet: sprite_sheet,
            image: image,
        }
    }

    pub fn update(&mut self) {
        self.state_machine = self.state_machine.update();
    }

    pub fn draw(&self, renderer: &Renderer) {
        let frame_name = format!(
            "{} ({}).png",
            self.state_machine.frame_name(),
            (self.state_machine.context().frame / 3) + 1
        );

        let sprite = self
            .sprite_sheet
            .frames
            .get(&frame_name)
            .expect("Cell not found");

        renderer.draw_image(
            &self.image,
            &Rect {
                x: sprite.frame.x.into(),
                y: sprite.frame.y.into(),
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            },
            &Rect {
                x: self.state_machine.context().position.x.into(),
                y: self.state_machine.context().position.y.into(),
                width: sprite.frame.w.into(),
                height: sprite.frame.h.into(),
            },
        )
    }
}

impl RedHatBoyStateMachine {
    fn frame_name(&self) -> &str {
        match self {
            RedHatBoyStateMachine::Idle(state) => state.frame_name(),
            RedHatBoyStateMachine::Running(state) => state.frame_name(),
        }
    }

    fn context(&self) -> &RedHatBoyContext {
        match self {
            RedHatBoyStateMachine::Idle(state) => &state.context(),
            RedHatBoyStateMachine::Running(state) => &state.context(),
        }
    }

    fn update(self) -> Self {
        match self {
            RedHatBoyStateMachine::Idle(mut state) => {
                if state.context.frame < 29 {
                    state.context.frame += 1;
                } else {
                    state.context.frame = 0;
                }
                RedHatBoyStateMachine::Idle(state)
            }
            RedHatBoyStateMachine::Running(_) => self,
        }
    }
}
