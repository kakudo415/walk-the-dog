mod rhb;

use crate::{
    browser,
    engine::{self, Game, KeyState, Rect, Renderer},
    game::rhb::RedHatBoy,
};
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub struct SheetRect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize, Clone)]
pub struct Cell {
    frame: SheetRect,
}
#[derive(Deserialize, Clone)]
pub struct Sheet {
    frames: HashMap<String, Cell>,
}

pub enum WalkTheDog {
    Loading,
    Loaded(RedHatBoy),
}

#[async_trait(?Send)]
impl Game for WalkTheDog {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        match self {
            WalkTheDog::Loading => {
                let json = browser::fetch_json("rhb.json").await?;
                let sheet = serde_wasm_bindgen::from_value::<Sheet>(json)
                    .expect("Could not parse rhb.json"); // json.into_serde() is deprecated
                let image = engine::load_image("rhb.png").await?;
                let rhb = RedHatBoy::new(sheet, image);
                Ok(Box::new(WalkTheDog::Loaded(rhb)))
            }
            WalkTheDog::Loaded(_) => Err(anyhow!("Error: Game is already initialized!")),
        }
    }

    fn update(&mut self, keystate: &KeyState) {
        if let WalkTheDog::Loaded(rhb) = self {
            if keystate.is_pressed("ArrowRight") {
                rhb.run_right();
            }
            if keystate.is_pressed("ArrowDown") {
                rhb.slide();
            }
            rhb.update();
        }
    }

    fn draw(&self, renderer: &Renderer) {
        renderer.clear(&Rect {
            x: 0.0,
            y: 0.0,
            width: 600.0,
            height: 600.0,
        });
        if let WalkTheDog::Loaded(rhb) = self {
            rhb.draw(renderer);
        }
    }
}

impl WalkTheDog {
    pub fn new() -> Self {
        WalkTheDog::Loading
    }
}
