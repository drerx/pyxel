use std::ffi::CString;
use std::mem;
use std::ptr;

use glow::Context;

use crate::sdl2_sys::*;

pub(crate) static mut WINDOW: *mut SDL_Window = ptr::null_mut();
static mut GL: *mut Context = ptr::null_mut();
//static mut RENDERER: *mut SDL_Renderer = ptr::null_mut();

pub fn gl() -> &'static mut Context {
    unsafe { &mut *GL }
}

pub(crate) fn init_window(title: &str, width: u32, height: u32) {
    unsafe {
        let title = CString::new(title).unwrap();
        WINDOW = SDL_CreateWindow(
            title.as_ptr(),
            SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            SDL_WINDOWPOS_UNDEFINED_MASK as i32,
            width as i32,
            height as i32,
            SDL_WINDOW_OPENGL,
        );
        if WINDOW.is_null() {
            panic!("Failed to create window");
        }
        /*RENDERER = SDL_CreateRenderer(WINDOW, -1, 0);
        if RENDERER.is_null() {
            panic!("Failed to create renderer");
        }*/
        if SDL_GL_CreateContext(WINDOW).is_null() {
            panic!("Failed to create OpenGL context");
        }
        GL = mem::transmute(Box::new(Context::from_loader_function(|s| {
            SDL_GL_GetProcAddress(s.as_ptr() as *const _) as *const _
        })));
    }

    /*
    let watch_info_file = Self::watch_info_file();
    let sdl_window = Self::load_watch_info(&watch_info_file).map_or_else(
        || {
            sdl_video
                .window(
                    title,
                    screen_width * display_scale,
                    screen_height * display_scale,
                )
                .position_centered()
                .resizable()
                .build()
                .unwrap()
        },
        |window_state| {
            sdl_video
                .window(title, window_state.width, window_state.height)
                .position(window_state.x, window_state.y)
                .resizable()
                .build()
                .unwrap()
        },
    );
    let mut sdl_canvas = sdl_window.into_canvas().present_vsync().build().unwrap();
    sdl_canvas
        .window_mut()
        .set_minimum_size(screen_width, screen_height)
        .unwrap();
    let sdl_texture = sdl_canvas
        .texture_creator()
        .create_texture_streaming(SdlPixelFormat::RGB24, screen_width, screen_height)
        .unwrap();
    let sdl_game_controller = sdl_context.game_controller().map_or_else(
        |_| {
            println!("Unable to initialize the game controller subsystem");
            None
        },
        |sdl_game_controller| Some(sdl_game_controller),
    );
    let sdl_audio = sdl_context.audio().map_or_else(
        |_| {
            println!("Unable to initialize the audio subsystem");
            None
        },
        |sdl_audio| Some(sdl_audio),
    );
    sdl_hint::set("SDL_MOUSE_FOCUS_CLICKTHROUGH", "1");
    Self::set_instance(Self {
        sdl_context,
        sdl_event_pump,
        sdl_timer,
        sdl_canvas,
        sdl_texture,
        sdl_game_controller,
        sdl_game_controller_states: Vec::new(),
        sdl_audio,
        sdl_audio_device: None,
        screen_width,
        screen_height,
        mouse_x: i32::MIN,
        mouse_y: i32::MIN,
        watch_info_file,
        window_state: WindowState::default(),
        #[cfg(target_os = "emscripten")]
        virtual_gamepad_states: [false; 8],
    });
    Self::instance().save_watch_info();
    */
}

pub fn swap_window() {
    unsafe {
        SDL_GL_SwapWindow(WINDOW);
    }
}

pub fn set_window_title(title: &str) {
    let title = CString::new(title).unwrap();
    unsafe {
        SDL_SetWindowTitle(WINDOW, title.as_ptr());
    }
}

pub fn set_window_icon(width: u32, height: u32, pixels: &[u32]) {
    /*
    let mut sdl_surface =
        SdlSurface::new(width * scale, height * scale, SdlPixelFormat::RGBA32).unwrap();
    let pitch = sdl_surface.pitch();
    sdl_surface.with_lock_mut(|buffer: &mut [u8]| {
        for y in 0..height * scale {
            for x in 0..width * scale {
                let color = image[(width * (y / scale) + x / scale) as usize];
                let rgb = colors[color as usize];
                let offset = (y * pitch + x * 4) as usize;
                buffer[offset] = ((rgb >> 16) & 0xff) as u8;
                buffer[offset + 1] = ((rgb >> 8) & 0xff) as u8;
                buffer[offset + 2] = (rgb & 0xff) as u8;
                buffer[offset + 3] = if color > 0 { 0xff } else { 0x00 };
            }
        }
    });
    self.sdl_canvas.window_mut().set_icon(&sdl_surface);
    */
}

pub fn window_pos() -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    unsafe {
        SDL_GetWindowPosition(WINDOW, &mut x as *mut i32, &mut y as *mut i32);
    }
    (x, y)
}

pub fn set_window_pos(x: i32, y: i32) {
    unsafe {
        SDL_SetWindowPosition(WINDOW, x, y);
    }
}

pub fn window_size() -> (u32, u32) {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    unsafe {
        SDL_GetWindowSize(WINDOW, &mut width as *mut i32, &mut height as *mut i32);
    }
    (width as u32, height as u32)
}

pub fn set_window_size(width: u32, height: u32) {
    unsafe {
        SDL_SetWindowSize(WINDOW, width as i32, height as i32);
    }
}

pub fn is_fullscreen() -> bool {
    (unsafe { SDL_GetWindowFlags(WINDOW) } & SDL_WINDOW_FULLSCREEN) != 0
}

pub fn set_fullscreen(fullscreen: bool) {
    let fullscreen = if fullscreen {
        SDL_WINDOW_FULLSCREEN_DESKTOP
    } else {
        0
    };
    unsafe {
        SDL_SetWindowFullscreen(WINDOW, fullscreen);
    }
}

pub fn display_size() -> (u32, u32) {
    let mut current = SDL_DisplayMode {
        format: 0,
        w: 0,
        h: 0,
        refresh_rate: 0,
        driverdata: std::ptr::null_mut(),
    };
    if unsafe { SDL_GetCurrentDisplayMode(0, &mut current as *mut SDL_DisplayMode) } != 0 {
        panic!("Failed to get display size");
    }
    (current.w as u32, current.h as u32)
}