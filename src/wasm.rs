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

#[unsafe(no_mangle)]
pub extern "C" fn wasm_app_new() -> *mut WasmApp {
    Box::into_raw(Box::new(WasmApp::new()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_app_free(ptr: *mut WasmApp) {
    if !ptr.is_null() {
        drop(Box::from_raw(ptr));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_app_next_button(ptr: *mut WasmApp) {
    if let Some(app) = ptr.as_mut() {
        app.app.next_button(&CARD);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_app_previous_button(ptr: *mut WasmApp) {
    if let Some(app) = ptr.as_mut() {
        app.app.previous_button(&CARD);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_app_tick(ptr: *mut WasmApp, now: u64) {
    if let Some(app) = ptr.as_mut() {
        app.app.clear_expired_dialogs(now);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_app_activate(
    ptr: *mut WasmApp,
    width: u16,
    height: u16,
    now: u64,
) -> i32 {
    let Some(app) = ptr.as_mut() else {
        return -1;
    };

    match app.app.activate_selected(&CARD) {
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
            app.app.spawn_lol_dialogs(area, lol_dialog_count(), now);
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_app_render(ptr: *mut WasmApp, width: u16, height: u16) -> *const u8 {
    let Some(app) = ptr.as_mut() else {
        return std::ptr::null();
    };

    let output = render_ansi(&app.app, &CARD, width, height);
    app.render_buffer = output.into_bytes();
    app.render_buffer.as_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_app_render_len(ptr: *mut WasmApp) -> usize {
    ptr.as_ref().map_or(0, |app| app.render_buffer.len())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasm_app_dialog_count(ptr: *mut WasmApp) -> usize {
    ptr.as_ref().map_or(0, |app| app.app.dialogs.len())
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_link_count() -> usize {
    CARD.links.len()
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_link_url_len(index: usize) -> usize {
    CARD.links.get(index).map_or(0, |link| link.url.len())
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_link_url_ptr(index: usize) -> *const u8 {
    CARD.links
        .get(index)
        .map_or(std::ptr::null(), |link| link.url.as_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn wasm_now_ms() -> u64 {
    now_ms()
}
