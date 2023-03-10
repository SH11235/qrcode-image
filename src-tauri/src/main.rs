// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use image::{ImageBuffer, Luma};
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use std::io::Cursor;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_qr_image(text: &str) -> Vec<u8> {
    let err_correction_level: QrCodeEcc = QrCodeEcc::Low;
    let qr_binary = QrCode::encode_binary(&text.as_bytes(), err_correction_level).unwrap();
    // エンコードした画像を返す
    let mut image_buffer = ImageBuffer::new(qr_binary.size() as u32, qr_binary.size() as u32);
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let color = if qr_binary.get_module(x as i32, y as i32) {
            Luma([0u8])
        } else {
            Luma([255u8])
        };
        *pixel = color;
    }
    let mut image_data = Vec::new();
    let mut cursor = Cursor::new(&mut image_data);
    image_buffer
        .write_to(&mut cursor, image::ImageOutputFormat::Png)
        .unwrap();
    image_data
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_qr_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
