use anyhow::Result;
use winit::keyboard::NamedKey;

use crate::{
    engine::State,
    graphics::{Canvas, Color, Point, Size, Sprite, SpriteFont},
    screen::{
        fade_in, fade_in_only, fade_out, fade_out_by_color, fade_out_only, screen, screen_at,
        screen_copy,
    },
    task::sleep,
};

macro_rules! cancel_fn {
    ($state:expr) => {
        || {
            let input = $state.input.borrow();

            input.key_pressed(NamedKey::Enter)
                || input.key_pressed(NamedKey::Escape)
                || input.key_pressed(NamedKey::Space)
        }
    };
}

macro_rules! try_ok {
    ($token:expr) => {
        if $token {
            return Ok(false);
        }
    };
}

pub async fn show_gremlin(state: &mut State) -> Result<bool> {
    const SPLASH_SIZE: Size = Size::wh(16, 8);

    let (q00, ref pal) = state.arc.get_with_palette("Q00")?;

    Sprite::from(q00).draw(screen(), pal);

    let cancel = cancel_fn!(state);
    try_ok!(fade_in(Some(&cancel)).await);
    try_ok!(sleep(200).with_cancel(&cancel).await);

    let stars = state
        .arc
        .get_series("Q01", SPLASH_SIZE.width * SPLASH_SIZE.height)?;

    for i in [0usize, 1, 2, 3, 2, 1, 0] {
        let timer = sleep(100).with_cancel(&cancel);

        Sprite::from(stars[i].to_vec())
            .with_size(SPLASH_SIZE)
            .draw(screen_at((112, 85)), pal);

        try_ok!(timer.await);
    }

    for i in [4usize, 5, 6, 7, 6, 5, 4] {
        let timer = sleep(100).with_cancel(&cancel);

        Sprite::from(stars[i].to_vec())
            .with_size(SPLASH_SIZE)
            .draw(screen_at((144, 110)), pal);

        try_ok!(timer.await);
    }

    try_ok!(fade_out(Some(&cancel)).await);

    Ok(true)
}

pub async fn show_magnetic_fields(state: &mut State) -> Result<bool> {
    const KEYS: [&str; 22] = [
        "Q02", "Q03", "Q04", "Q05", "Q06", "Q07", "Q08", "Q09", "Q0A", "Q0B", "Q0C", "Q0D", "Q0E",
        "Q0F", "Q10", "Q11", "Q12", "Q13", "Q14", "Q15", "Q16", "Q17",
    ];

    let cancel = cancel_fn!(state);

    let (_, ref pal) = state.arc.get_with_palette(KEYS.last().unwrap())?;

    for key in KEYS {
        let timer = sleep(50).with_cancel(&cancel);

        Sprite::from(state.arc.get(key)?).draw(screen(), pal);

        try_ok!(timer.await);
    }

    try_ok!(sleep(1000).with_cancel(&cancel).await);
    try_ok!(fade_out(Some(&cancel)).await);

    Ok(true)
}

pub async fn show_credits(state: &mut State) -> Result<bool> {
    const CREDITS_FADE_IN_TIMEOUT: u64 = 2000;
    const CREDITS_FADE_OUT_TIMEOUT: u64 = 1000;
    const CREDITS: [Option<(&str, u32, u32)>; 26] = [
        Some(("A GAME", 118, 43)),
        Some(("BY", 146, 67)),
        Some(("ANDREW MORRIS", 69, 91)),
        Some(("AND", 139, 115)),
        Some(("SHAUN SOUTHERN", 62, 139)),
        None,
        Some(("LEVEL DESIGN", 76, 67)),
        Some(("BY", 146, 91)),
        Some(("PETER LIGGETT", 69, 115)),
        None,
        Some(("MUSIC", 125, 67)),
        Some(("BY", 146, 91)),
        Some(("PATRICK PHELAN", 62, 115)),
        None,
        Some(("PC CONVERSION", 69, 43)),
        Some(("BY", 146, 67)),
        Some(("JON MEDHURST FOR", 48, 91)),
        Some(("CYGNUS SOFTWARE", 55, 115)),
        Some(("ENGINEERING LTD.", 52, 139)),
        None,
        Some(("COPYRIGHT 1993", 62, 43)),
        Some(("MAGNETIC FIELDS", 55, 67)),
        Some(("(SOFTWARE DESIGN) LTD.", 10, 91)),
        Some(("GREMLIN GRAPHICS", 48, 115)),
        Some(("SOFTWARE LTD.", 73, 139)),
        None,
    ];

    let (q19, ref pal) = state.arc.get_with_palette("Q19")?;

    let bgr = Sprite::from(q19);

    bgr.draw(screen(), pal);

    let cancel = cancel_fn!(state);
    try_ok!(fade_in(Some(&cancel)).await);
    try_ok!(sleep(2000).with_cancel(&cancel).await);

    let (back, mut front) = (screen_copy(), Canvas::new());

    let font = SpriteFont::from(state.arc.get("Q1A")?);

    for item in CREDITS {
        match item {
            Some((text, x, y)) => {
                font.print(front.raw_at((x, y)), text);
            }
            None => {
                try_ok!(fade_in_only(&back, &front, Some(&cancel)).await);
                try_ok!(sleep(CREDITS_FADE_IN_TIMEOUT).with_cancel(&cancel).await);

                try_ok!(fade_out_only(&back, &front, Some(&cancel)).await);
                try_ok!(sleep(CREDITS_FADE_OUT_TIMEOUT).with_cancel(&cancel).await);

                front = Canvas::new();
            }
        }
    }

    let q1b = state.arc.get("Q1B")?;

    for step in 1..=36 {
        let timer = sleep(50).with_cancel(&cancel);

        bgr.draw(screen(), pal);
        draw_a_car(screen(), &q1b, pal, step);

        try_ok!(timer.await);
    }

    for key in ["Q1C", "Q1D"] {
        let timer = sleep(50).with_cancel(&cancel);

        Sprite::from(state.arc.get(key)?).draw(screen(), pal);

        try_ok!(timer.await);
    }

    let q1e = state.arc.get("Q1E")?;
    let color_ix = q1e[0] as usize;

    Sprite::from(q1e).draw(screen(), pal);

    try_ok!(sleep(2000).with_cancel(&cancel).await);

    let color = pal
        .get(color_ix * 3..color_ix * 3 + 3)
        .map(|rgb| Color::rgb(rgb[0] << 2, rgb[1] << 2, rgb[2] << 2));

    try_ok!(fade_out_by_color(color.unwrap(), Some(&cancel)).await);
    try_ok!(sleep(4000).with_cancel(&cancel).await);
    try_ok!(fade_out(Some(&cancel)).await);

    Ok(true)
}

fn draw_a_car(canvas: &mut [u32], data: &[u8], pal: &[u8], step: usize) {
    const WIDTH: usize = 336; // NB: not 320!

    let cx = 256 + (36 - step) * 512;
    let di = 160 - ((((WIDTH * 170) + 224) / cx) >> 1) + (WIDTH * 64)
        - (((((WIDTH * 118) + 288) / cx) * (WIDTH * 32 + 170)) >> 16) * WIDTH;

    let xx = di % WIDTH;
    let mut y = di / WIDTH;

    for row in (0..WIDTH * 118).step_by(cx) {
        let offset = (row >> 8) * 224;
        let mut x = xx;

        for i in (offset..offset + 224).step_by(cx >> 8) {
            let px = data[i] as usize;

            if px != 0xFF {
                canvas[Point::xy(x as u32, y as u32).index()] = u32::from_be_bytes([
                    255,
                    pal[px * 3] << 2,
                    pal[px * 3 + 1] << 2,
                    pal[px * 3 + 2] << 2,
                ]);
            }

            x += 1;
        }

        y += 1;
    }
}

pub async fn show_lotus_logo(state: &mut State) -> Result<bool> {
    let (q18, ref pal) = state.arc.get_with_palette("Q18")?;

    Sprite::from(q18).draw(screen(), pal);

    let cancel = cancel_fn!(state);
    try_ok!(fade_in(Some(&cancel)).await);
    try_ok!(sleep(2000).with_cancel(&cancel).await);
    try_ok!(fade_out(Some(&cancel)).await);

    Ok(true)
}

pub async fn show_magazine(state: &mut State) -> Result<bool> {
    const VIDEO_SIZE: Size = Size::wh(160, 112);
    const VIDEO_POS: Point = Point::xy(136, 38);

    let (v32, ref pal) = state.arc.get_with_palette("V32")?;

    fn get_with_leading_pal(mut raw: Vec<u8>, pal: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let dat = raw.split_off(720);

        let mut pal = pal.to_vec();
        let _ = pal.splice(16 * 3.., raw);

        Ok((dat, pal))
    }

    const KEYS: [&str; 41] = [
        "V00", "V01", "V02", "V03", "V04", "V05", "V06", "V07", "V08", "V09", "V0A", "V0B", "V0C",
        "V0D", "V0E", "V0F", "V10", "V11", "V12", "V13", "V14", "V15", "V16", "V17", "V18", "V19",
        "V1A", "V1B", "V1C", "V1D", "V1E", "V1F", "V20", "V21", "V22", "V23", "V24", "V25", "V26",
        "V27", "V28",
    ];

    let mut back = Canvas::new();
    let mut front = Canvas::new();

    let bgr = Sprite::from(v32);
    bgr.draw(back.raw(), pal);
    bgr.draw(screen(), pal);

    let cancel = cancel_fn!(state);

    for key in KEYS {
        let timer = sleep(100).with_cancel(&cancel);

        let (vxx, ref pal) = get_with_leading_pal(state.arc.get(key)?, pal)?;
        let vxx = Sprite::from(vxx).with_size(VIDEO_SIZE);

        vxx.draw(screen_at(VIDEO_POS), pal);

        if Some(&key) == KEYS.first() {
            try_ok!(fade_in(Some(&cancel)).await);
        } else {
            // save the last frame
            if Some(&key) == KEYS.last() {
                vxx.draw(front.raw_at(VIDEO_POS), pal);
            }
        }

        try_ok!(timer.await);
    }

    try_ok!(fade_out_only(&back, &front, Some(&cancel)).await);

    let (v33, ref pal) = get_with_leading_pal(state.arc.get("V33")?, pal)?;

    Sprite::from(v33)
        .with_size(VIDEO_SIZE)
        .draw(front.raw_at(VIDEO_POS), pal);

    try_ok!(fade_in_only(&back, &front, Some(&cancel)).await);
    try_ok!(sleep(2000).with_cancel(&cancel).await);
    try_ok!(fade_out(Some(&cancel)).await);

    Ok(true)
}
