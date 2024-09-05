use anyhow::Result;
use std::cell::Ref;
use winit::keyboard::NamedKey;

use super::{FRAME_OFFSET, FRAME_SIZE_ST, MENU_ITEM_SIZE};
use crate::{
    engine::State,
    game::define_menu,
    game::options::{Acceleration, Config, Course, Race, Transmission},
    graphics::{
        font::{Font, CHAR_SET_03, CHAR_SET_04},
        Frame, Sprite, FRAME_BORDER,
    },
    input::{InputHelper, BACKSPACE_CHAR},
    screen::{fade_in, fade_out, screen, screen_at},
    task::yield_now,
};

struct Position {
    row: u8,
    col: u8,
    editor: bool,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            row: 0,
            col: 1,
            editor: false,
        }
    }
}

pub enum Action {
    Start,
    Exit,
}

pub async fn main_menu(state: &mut State) -> Result<Action> {
    let (i14, ref pal) = state.arc.get_with_palette("I14")?;

    let bgr = Sprite::from(i14);

    let i15 = state
        .arc
        .get_series("I15", MENU_ITEM_SIZE.width * MENU_ITEM_SIZE.height)?;

    let trans = [
        Sprite::from(i15.first().unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Transmission::Manual
        Sprite::from(i15.get(1).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Transmission::Automatic
    ];

    let accel = [
        Sprite::from(i15.get(2).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Acceleration::Button
        Sprite::from(i15.get(3).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Acceleration::Joystick
    ];

    let race = [
        Sprite::from(i15.get(6).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Race::TimeLimit
        Sprite::from(i15.get(7).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Race::Competition
    ];

    let player = [
        Sprite::from(i15.get(8).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // 1 player
        Sprite::from(i15.get(9).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // 2 players
    ];

    let course = [
        Sprite::from(i15.get(10).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Course::T1
        Sprite::from(i15.get(11).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Course::T2
        Sprite::from(i15.get(12).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Course::T3
        Sprite::from(i15.get(13).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Course::Circular
        Sprite::from(i15.get(14).unwrap().to_vec()).with_size(MENU_ITEM_SIZE), // Course::Unknown
    ];

    let font_c03 = Font::from(CHAR_SET_03, state.arc.get("C03")?);
    let font_c04 = Font::from(CHAR_SET_04, state.arc.get("C04")?);
    let frame = Frame::new(FRAME_SIZE_ST);

    let mut first_time = true;
    let mut pos = Position::default();

    let action = loop {
        yield_now().await;

        let (key_pressed, action, menu) =
            handle_input(state.input.borrow(), &mut pos, &mut state.cfg);

        if let Some(action) = action {
            break action;
        }

        if let Some(menu) = menu {
            first_time = true;
            fade_out(None).await;

            match menu {
                Menu::Define => define_menu(state, pal).await?,
            }
        }

        if first_time || key_pressed {
            bgr.draw(screen(), pal);

            trans[state.cfg.p1_trans as usize].draw(screen_at((6, 52)), pal);
            accel[state.cfg.p1_accel as usize].draw(screen_at((6, 91)), pal);
            trans[state.cfg.p2_trans as usize].draw(screen_at((214, 52)), pal);
            accel[state.cfg.p2_accel as usize].draw(screen_at((214, 91)), pal);
            race[state.cfg.race as usize].draw(screen_at((110, 52)), pal);
            course[state.cfg.course as usize].draw(screen_at((110, 91)), pal);

            player[state.cfg.players_num as usize - 1].draw(screen_at((110, 130)), pal);

            font_c04.print(screen_at((13, 21)), &state.cfg.p1_name, pal);
            font_c04.print(screen_at((221, 21)), &state.cfg.p2_name, pal);
            font_c03.print(screen_at((117, 177)), &state.cfg.code, pal);

            frame.draw(
                screen_at((
                    (pos.col as u32 * (frame.size.width - FRAME_BORDER + 1) + FRAME_OFFSET.0),
                    (pos.row as u32 * (frame.size.height - FRAME_BORDER + 1) + FRAME_OFFSET.1),
                )),
                pal,
            );

            if first_time {
                first_time = false;

                fade_in(None).await;
            }
        }
    };

    fade_out(None).await;

    Ok(action)
}

macro_rules! switch_option {
    ($opt:expr, $v1:expr, $v2:expr) => {
        $opt = if $opt == $v1 { $v2 } else { $v1 }
    };
    ($opt:expr, $v1:expr, $v2:expr, $v3:expr, $v4:expr, $v5:expr) => {
        $opt = if $opt == $v1 {
            $v2
        } else if $opt == $v2 {
            $v3
        } else if $opt == $v3 {
            $v4
        } else if $opt == $v4 {
            $v5
        } else {
            $v1
        }
    };
}

enum Menu {
    Define,
}

fn handle_input(
    input: Ref<InputHelper>,
    pos: &mut Position,
    cfg: &mut Config,
) -> (bool, Option<Action>, Option<Menu>) {
    let mut key_pressed = false;
    let mut action = None;
    let mut menu = None;

    if input.key_pressed(NamedKey::ArrowUp) && pos.row > 0 {
        pos.row -= 1;
        pos.editor = false;
        key_pressed = true;
    }

    if input.key_pressed(NamedKey::ArrowDown) && pos.row < 4 {
        pos.row += 1;
        pos.editor = false;
        key_pressed = true;
    }

    if input.key_pressed(NamedKey::ArrowLeft) && pos.col > 0 {
        pos.col -= 1;
        pos.editor = false;
        key_pressed = true;
    }

    if input.key_pressed(NamedKey::ArrowRight) && pos.col < 2 {
        pos.col += 1;
        pos.editor = false;
        key_pressed = true;
    }

    if input.key_pressed(NamedKey::Escape) {
        if pos.editor {
            pos.editor = false;
        } else {
            action = Some(Action::Exit);
        }

        key_pressed = true;
    }

    if input.key_pressed(NamedKey::Enter) {
        match (pos.row, pos.col) {
            (0, 0) => {
                pos.editor = !pos.editor;
                key_pressed = true;
            }
            (0, 1) => {
                action = Some(Action::Start);
                key_pressed = true;
            }
            (0, 2) => {
                pos.editor = !pos.editor;
                key_pressed = true;
            }
            (1, 0) => {
                switch_option!(cfg.p1_trans, Transmission::Manual, Transmission::Automatic);
                key_pressed = true;
            }
            (1, 1) => {
                switch_option!(cfg.race, Race::TimeLimit, Race::Competition);
                key_pressed = true;
            }
            (1, 2) => {
                switch_option!(cfg.p2_trans, Transmission::Manual, Transmission::Automatic);
                key_pressed = true;
            }
            (2, 0) => {
                switch_option!(cfg.p1_accel, Acceleration::Button, Acceleration::Joystick);
                key_pressed = true;
            }
            (2, 1) => {
                switch_option!(
                    cfg.course,
                    Course::T1,
                    Course::T2,
                    Course::T3,
                    Course::Circular,
                    Course::Unknown
                );
                key_pressed = true;
            }
            (2, 2) => {
                switch_option!(cfg.p2_accel, Acceleration::Button, Acceleration::Joystick);
                key_pressed = true;
            }
            (3, 0) => {
                // Controls
                key_pressed = true;
            }
            (3, 1) => {
                switch_option!(cfg.players_num, 1, 2);
                key_pressed = true;
            }
            (3, 2) => {
                // Sound Settings
                key_pressed = true;
            }
            (4, 0) => {
                // RECS
                key_pressed = true;
            }
            (4, 1) => {
                pos.editor = !pos.editor;
                key_pressed = true;
            }
            (4, 2) => {
                menu = Some(Menu::Define);
                key_pressed = true;
            }
            _ => {}
        }
    }

    if pos.editor {
        for c in input.chars() {
            match c {
                _ if c.is_ascii_alphabetic() || c.is_ascii_digit() => match (pos.row, pos.col) {
                    (0, 0) if cfg.p1_name.len() < 12 => {
                        cfg.p1_name.push(c.to_ascii_uppercase());
                        key_pressed = true;
                    }
                    (0, 2) if cfg.p2_name.len() < 12 => {
                        cfg.p2_name.push(c.to_ascii_uppercase());
                        key_pressed = true;
                    }
                    (4, 1) if cfg.code.len() < 12 => {
                        cfg.code.push(c.to_ascii_uppercase());
                        key_pressed = true;
                    }
                    _ => {}
                },
                BACKSPACE_CHAR => match (pos.row, pos.col) {
                    (0, 0) => {
                        cfg.p1_name.pop();
                        key_pressed = true;
                    }
                    (0, 2) => {
                        cfg.p2_name.pop();
                        key_pressed = true;
                    }
                    (4, 1) => {
                        cfg.code.pop();
                        key_pressed = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    (key_pressed, action, menu)
}
