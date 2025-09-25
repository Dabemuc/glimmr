use crate::args::{filetypes::Filetype, themes::Theme};
use crate::fs_parser::fs_structs::FlatFsEntry;
use std::path::PathBuf;
mod svg_helper;
use log::debug;
use svg_helper::compose_svg_from_filestruct;

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::{
    CaptureScreenshotFormat, CaptureScreenshotParams, Viewport,
};
use futures::StreamExt;
use tokio::runtime::Builder;

pub fn visualize(
    filestructure: Vec<FlatFsEntry>,
    theme: Theme,
    filetype: Filetype,
    output_filepath: PathBuf,
    bake_font: bool,
) {
    match filetype {
        Filetype::SVG => build_svg(
            filestructure,
            theme,
            output_filepath,
            Filetype::SVG.extension(),
            bake_font,
        ),
        Filetype::PNG => build_png(
            filestructure,
            theme,
            output_filepath,
            Filetype::PNG.extension(),
        ),
    }
}

fn build_svg(
    filestructure: Vec<FlatFsEntry>,
    theme: Theme,
    mut output_filepath: PathBuf,
    extension: &'static str,
    bake_font: bool,
) {
    // Compose svg
    let document = compose_svg_from_filestruct(filestructure, theme, bake_font);

    // Output
    debug!("Provided output_filepath: {}", output_filepath.display());
    if output_filepath.extension().is_none() {
        output_filepath.set_extension(extension);
        debug!("After adding extension: {}", output_filepath.display());
    }
    svg::save(output_filepath, &document).unwrap();
}

pub fn build_png(
    filestructure: Vec<FlatFsEntry>,
    theme: Theme,
    mut output_filepath: PathBuf,
    extension: &'static str,
) {
    // Compose SVG (always bake font for PNG rendering)
    let document = compose_svg_from_filestruct(filestructure, theme, true);
    let svg_data = document.to_string();

    // Run Chromium in a Tokio runtime
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(async {
        // Launch browser
        let (browser, mut handler) =
            Browser::launch(BrowserConfig::builder().no_sandbox().build().unwrap())
                .await
                .unwrap();

        // Spawn event handler
        tokio::spawn(async move {
            while let Some(_event) = handler.next().await {
                // Process browser events if needed
            }
        });

        // New blank page
        let page = browser.new_page("about:blank").await.unwrap();

        // Load SVG as content
        page.set_content(svg_data).await.unwrap();

        // Let JS run
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // Get bounding box of the SVG
        let element = page.find_element("svg").await.unwrap();
        let bbox_result = element
            .call_js_fn("function() { return this.getBoundingClientRect(); }", true)
            .await
            .unwrap();

        // Convert to JSON value
        let bbox: serde_json::Value = serde_json::from_value(bbox_result.result.value.unwrap())
            .expect("Failed to parse bounding box");

        let x = bbox["x"].as_f64().unwrap();
        let y = bbox["y"].as_f64().unwrap();
        let width = bbox["width"].as_f64().unwrap();
        let height = bbox["height"].as_f64().unwrap();

        // Screenshot with clip matching the SVG bounding box
        // Screenshot with clip matching the SVG bounding box
        let png_bytes = page
            .screenshot(CaptureScreenshotParams {
                format: Some(CaptureScreenshotFormat::Png),
                quality: None, // Only used for JPEG
                clip: Some(Viewport {
                    x,
                    y,
                    width,
                    height,
                    scale: 1.0,
                }),
                capture_beyond_viewport: Some(true), // ensures nothing is cut off
                from_surface: Some(true),
                optimize_for_speed: Some(false),
            })
            .await
            .unwrap();

        // Fix extension if missing
        if output_filepath.extension().is_none() {
            output_filepath.set_extension(extension);
        }

        // Save PNG
        std::fs::write(&output_filepath, png_bytes).unwrap();
        debug!("Saved PNG to {}", output_filepath.display());
    });
}
