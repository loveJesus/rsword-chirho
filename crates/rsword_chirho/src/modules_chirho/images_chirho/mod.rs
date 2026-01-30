// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Image and map module support.
//!
//! This module provides support for SWORD image/map modules,
//! which contain binary image data with optional coordinate overlays
//! for interactive maps.

mod image_module_chirho;

pub use image_module_chirho::{
    ImageModuleChirho, ImageEntryChirho, ImageFormatChirho,
    HotspotChirho, HotspotShapeChirho, MapOverlayChirho,
};
