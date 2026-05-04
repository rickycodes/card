use crate::app::{lol_dialog_count, now_ms, render_ansi, ActivateResult, AppState};
use crate::constants::CARD;

pub struct WasmApp {
    app: AppState,
    render_buffer: Vec<u8>,
}

impl WasmApp {
    fn new() -> Self {
        Self {
            app: AppState::new(),
            render_buffer: Vec::new(),
        }
    }
}

#[no_mangle]
pub extern "C" fn wasm_app_new() -> *mut WasmApp {
    Box::into_raw(Box::new(WasmApp::new()))
}

#[no_mangle]
pub unsafe extern "C" fn wasm_app_free(ptr: *mut WasmApp) {
    if !ptr.is_null() {
        // SAFETY: `ptr` was created by `Box::into_raw` in `wasm_app_new`, and
        // the null case is filtered out above.
        unsafe {
            drop(Box::from_raw(ptr));
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wasm_app_next_button(ptr: *mut WasmApp) {
    if !ptr.is_null() {
        // SAFETY: `ptr` is non-null and was allocated by `wasm_app_new`.
        unsafe {
            (*ptr).app.next_button(&CARD);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wasm_app_previous_button(ptr: *mut WasmApp) {
    if !ptr.is_null() {
        // SAFETY: `ptr` is non-null and was allocated by `wasm_app_new`.
        unsafe {
            (*ptr).app.previous_button(&CARD);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wasm_app_tick(ptr: *mut WasmApp, now: u64) {
    if !ptr.is_null() {
        // SAFETY: `ptr` is non-null and was allocated by `wasm_app_new`.
        unsafe {
            (*ptr).app.clear_expired_dialogs(now);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wasm_app_activate(
    ptr: *mut WasmApp,
    width: u16,
    height: u16,
    now: u64,
) -> i32 {
    if ptr.is_null() {
        return -1;
    }

    // SAFETY: `ptr` is non-null and was allocated by `wasm_app_new`.
    unsafe {
        match (*ptr).app.activate_selected(&CARD) {
            ActivateResult::OpenUrl(url) => CARD
                .links
                .iter()
                .position(|link| link.url == url)
                .map(|index| index as i32)
                .unwrap_or(-1),
            ActivateResult::SpawnLol => {
                let area = ratatui::prelude::Rect {
                    x: 0,
                    y: 0,
                    width,
                    height,
                };
                (*ptr).app.spawn_lol_dialogs(area, lol_dialog_count(), now);
                -1
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wasm_app_render(ptr: *mut WasmApp, width: u16, height: u16) -> *const u8 {
    if ptr.is_null() {
        return std::ptr::null();
    }

    // SAFETY: `ptr` is non-null and was allocated by `wasm_app_new`.
    unsafe {
        let output = render_ansi(&(*ptr).app, &CARD, width, height);
        (*ptr).render_buffer = output.into_bytes();
        (*ptr).render_buffer.as_ptr()
    }
}

#[no_mangle]
pub unsafe extern "C" fn wasm_app_render_len(ptr: *mut WasmApp) -> usize {
    if ptr.is_null() {
        0
    } else {
        // SAFETY: `ptr` is non-null and was allocated by `wasm_app_new`.
        unsafe { (*ptr).render_buffer.len() }
    }
}

#[no_mangle]
pub unsafe extern "C" fn wasm_app_dialog_count(ptr: *mut WasmApp) -> usize {
    if ptr.is_null() {
        0
    } else {
        // SAFETY: `ptr` is non-null and was allocated by `wasm_app_new`.
        unsafe { (*ptr).app.dialogs.len() }
    }
}

#[no_mangle]
pub extern "C" fn wasm_link_count() -> usize {
    CARD.links.len()
}

#[no_mangle]
pub extern "C" fn wasm_link_url_len(index: usize) -> usize {
    CARD.links.get(index).map_or(0, |link| link.url.len())
}

#[no_mangle]
pub extern "C" fn wasm_link_url_ptr(index: usize) -> *const u8 {
    CARD.links
        .get(index)
        .map_or(std::ptr::null(), |link| link.url.as_ptr())
}

#[no_mangle]
pub extern "C" fn wasm_now_ms() -> u64 {
    now_ms()
}
