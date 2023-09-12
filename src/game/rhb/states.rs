use crate::engine::Point;

const FLOOR: i16 = 475;

#[derive(Copy, Clone)]
pub struct RedHatBoyState<S> {
    context: RedHatBoyContext,
    _state: S,
}

#[derive(Copy, Clone)]
pub struct RedHatBoyContext {
    pub frame: u8,
    pub position: Point,
    pub velocity: Point,
}

const RUNNING_SPEED: i16 = 3;

impl RedHatBoyContext {
    pub fn update(mut self, frame_count: u8) -> Self {
        if self.frame < frame_count {
            self.frame += 1;
        } else {
            self.frame = 0;
        }
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self
    }

    fn reset_frame(mut self) -> Self {
        self.frame = 0;
        self
    }

    pub fn run_right(mut self) -> Self {
        self.velocity.x = RUNNING_SPEED;
        self
    }
}

#[derive(Copy, Clone)]
pub struct Idle;

#[derive(Copy, Clone)]
pub struct Running;

impl<S> RedHatBoyState<S> {
    pub fn context(&self) -> &RedHatBoyContext {
        &self.context
    }
}

const IDLE_FRAMES: u8 = 29;
const RUNNING_FRAMES: u8 = 23;

impl RedHatBoyState<Idle> {
    pub fn new() -> Self {
        RedHatBoyState {
            context: RedHatBoyContext {
                frame: 0,
                position: Point { x: 0, y: FLOOR },
                velocity: Point { x: 0, y: 0 },
            },
            _state: Idle {},
        }
    }

    pub fn run(self) -> RedHatBoyState<Running> {
        RedHatBoyState {
            context: self.context.reset_frame().run_right(),
            _state: Running {},
        }
    }

    pub fn frame_name(&self) -> &str {
        "Idle"
    }

    pub fn update(&mut self) {
        self.context = self.context.update(IDLE_FRAMES);
    }
}

impl RedHatBoyState<Running> {
    pub fn frame_name(&self) -> &str {
        "Run"
    }

    pub fn update(&mut self) {
        self.context = self.context.update(RUNNING_FRAMES);
    }
}
