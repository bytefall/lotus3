use anyhow::Result;
use winit::keyboard::{Key, NamedKey};

use crate::{
    engine::State,
    game::options::Model,
    graphics::{Point, Size, Sprite},
    screen::{fade_in, fade_out, screen, screen_at},
    task::yield_now,
};

const ANIM_DELAY: u64 = 100;
const ANIM_SIZE: Size = Size::wh(88, 24); // 16 icons of 88x24 bytes each
const ANIM_POS: Point = Point::xy(91, 97);

const KEYS: &[(&str, &str)] = &[
    ("I1E", "I1D"), // Esprit S4
    ("I11", "I10"), // Elan SE
    ("I13", "I12"), // M200
];

pub async fn select_model(state: &mut State) -> Result<Option<Model>> {
    let mut model = Model::default();

    let mut sprites: Option<(Sprite, Vec<Sprite>, Vec<u8>)> = None;
    let mut frame: Option<usize> = None;
    let mut fade = false;

    let selection = 'main: loop {
        yield_now().await;

        for k in state.input.borrow().keys() {
            match k {
                Key::Named(NamedKey::ArrowLeft) => {
                    model = model.prev();
                    sprites = None;
                    fade = true;
                }
                Key::Named(NamedKey::ArrowRight) => {
                    model = model.next();
                    sprites = None;
                    fade = true;
                }
                Key::Named(NamedKey::Enter) => break 'main Some(model),
                Key::Named(NamedKey::Escape) => break 'main None,
                _ => {}
            }
        }

        if sprites.is_none() {
            let (bgr_key, ani_key) = KEYS[model as usize];
            let (bgr, pal) = state.arc.get_with_palette(bgr_key)?;
            let anim = state
                .arc
                .get_series(ani_key, ANIM_SIZE.width * ANIM_SIZE.height)?
                .into_iter()
                .map(|x| Sprite::from(x).with_size(ANIM_SIZE))
                .collect();

            frame = None;
            sprites = Some((Sprite::from(bgr), anim, pal));
        }

        if fade {
            fade = false;

            fade_out(None).await;
        }

        let Some((bgr, anim, pal)) = &sprites else {
            break None;
        };

        bgr.draw(screen(), pal);

        match frame {
            Some(ref mut i) => {
                std::thread::sleep(std::time::Duration::from_millis(ANIM_DELAY));

                anim[*i].draw(screen_at(ANIM_POS), pal);
                *i += 1;

                if *i == anim.len() {
                    *i = 0;
                }
            }
            None => {
                anim[0].draw(screen_at(ANIM_POS), pal);
                frame = Some(1);

                fade_in(None).await;
            }
        }
    };

    fade_out(None).await;

    Ok(selection)
}
