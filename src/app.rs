use anyhow::Result;
use softbuffer::Surface;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll, Waker},
    time::{Duration, Instant},
};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, ModifiersState},
    window::{Fullscreen, Window, WindowBuilder},
};

use crate::{
    graphics::{SCREEN_HEIGHT, SCREEN_WIDTH},
    input::InputHelper,
    screen::{get_screen_state, screen, set_screen_state},
    task::Signal,
};

pub struct Application {
    window: Rc<Window>,
    event_loop: EventLoop<()>,
    surface: Surface<Rc<Window>, Rc<Window>>,
    scale: u32,
    input: Rc<RefCell<InputHelper>>,
}

const DEFAULT_DELAY: Duration = Duration::from_millis(1);
const SCREEN_REDRAW: Duration = Duration::from_millis(1000 / 30);

impl Application {
    pub fn new(title: &str) -> Result<Self> {
        let event_loop = EventLoop::new()?;

        let scale = 3;
        let size = PhysicalSize::new(SCREEN_WIDTH * scale, SCREEN_HEIGHT * scale);

        let window = Rc::new(
            WindowBuilder::new()
                .with_title(title)
                .with_resizable(false)
                .with_inner_size(size)
                .build(&event_loop)?,
        );

        window.set_cursor_visible(false);

        let size = window.inner_size();
        let scale = (size.width / SCREEN_WIDTH).min(size.height / SCREEN_HEIGHT);

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let mut surface = Surface::new(&context, window.clone()).unwrap();

        surface
            .resize(size.width.try_into()?, size.height.try_into()?)
            .unwrap();

        Ok(Self {
            window,
            event_loop,
            surface,
            scale,
            input: Rc::new(RefCell::new(InputHelper::new())),
        })
    }

    pub fn run(
        mut self,
        mut step: impl FnMut(&mut Context<'static>, &Signal) -> Result<Poll<()>> + 'static,
    ) -> Result<()> {
        let signal = Arc::new(Signal::new());
        let waker = Waker::from(Arc::clone(&signal));
        let mut ctx = Context::from_waker(Box::leak(Box::new(waker)));

        let mut last_redraw = Instant::now();
        let mut modifiers = ModifiersState::default();
        let mut fullscreen = false;

        self.event_loop.run(move |event, elwt| {
            match &event {
                Event::NewEvents(_) => {
                    self.input.as_ref().borrow_mut().clear();
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::ModifiersChanged(new) => {
                        modifiers = new.state();
                    }
                    WindowEvent::Resized(size) => {
                        self.surface
                            .resize(
                                size.width.try_into().unwrap(),
                                size.height.try_into().unwrap(),
                            )
                            .unwrap();

                        self.scale = (size.width / SCREEN_WIDTH).min(size.height / SCREEN_HEIGHT);
                    }
                    WindowEvent::RedrawRequested => {
                        let mut buf = self.surface.buffer_mut().unwrap();

                        let mut dst = buf.iter_mut();
                        let mut src = screen().chunks_exact(SCREEN_WIDTH as usize).flat_map(|y| {
                            (0..self.scale)
                                .flat_map(|_| y.iter().flat_map(|x| (0..self.scale).map(|_| *x)))
                        });

                        while let (Some(dst), Some(src)) = (dst.next(), src.next()) {
                            *dst = src;
                        }

                        buf.present().unwrap();

                        set_screen_state();
                        last_redraw = Instant::now();
                    }
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Character(c),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } if modifiers.alt_key() => match c.as_str() {
                        c @ ("=" | "-") if !fullscreen => {
                            let scale = match (c, self.scale) {
                                ("=", s @ ..=3) => s + 1,
                                ("-", s @ 2..) => s - 1,
                                (_, _) => return,
                            };

                            let Some(mon) = self.window.current_monitor() else {
                                return;
                            };

                            let size =
                                PhysicalSize::new(SCREEN_WIDTH * scale, SCREEN_HEIGHT * scale);

                            if size < mon.size() {
                                if let Some(size) = self.window.request_inner_size(size) {
                                    self.surface
                                        .resize(
                                            size.width.try_into().unwrap(),
                                            size.height.try_into().unwrap(),
                                        )
                                        .unwrap();

                                    self.scale = scale;
                                }
                            }
                        }
                        "f" => {
                            fullscreen = !fullscreen;

                            self.window
                                .set_fullscreen(fullscreen.then_some(Fullscreen::Borderless(None)));
                        }
                        _ => (),
                    },
                    WindowEvent::KeyboardInput { event, .. } => {
                        self.input.borrow_mut().handle(event.clone());
                    }
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    let result = step(&mut ctx, &signal);

                    match result {
                        Ok(Poll::Pending) => {
                            if get_screen_state() && last_redraw.elapsed() >= SCREEN_REDRAW {
                                self.window.request_redraw();
                            } else {
                                elwt.set_control_flow(ControlFlow::wait_duration(DEFAULT_DELAY));
                            }
                        }
                        Ok(Poll::Ready(())) => elwt.exit(),
                        Err(e) => {
                            eprintln!("{e:?}");

                            elwt.exit();
                        }
                    }
                }
                _ => {}
            };
        })?;

        Ok(())
    }

    pub fn input(&self) -> Rc<RefCell<InputHelper>> {
        Rc::clone(&self.input)
    }
}
