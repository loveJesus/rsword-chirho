// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Image module driver implementation.
//!
//! Handles binary image modules with coordinate overlays for maps.
//! SWORD image modules store images in a directory structure with
//! optional hotspot definitions for interactive regions.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

use crate::config_chirho::ModuleConfigChirho;
use crate::error_chirho::{ErrorChirho, ResultChirho};

/// Image format types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormatChirho {
    /// JPEG image.
    JpegChirho,
    /// PNG image.
    PngChirho,
    /// GIF image.
    GifChirho,
    /// BMP image.
    BmpChirho,
    /// TIFF image.
    TiffChirho,
    /// Unknown format.
    UnknownChirho,
}

impl ImageFormatChirho {
    /// Detect format from file extension.
    pub fn from_extension_chirho(ext_chirho: &str) -> Self {
        match ext_chirho.to_lowercase().as_str() {
            "jpg" | "jpeg" => Self::JpegChirho,
            "png" => Self::PngChirho,
            "gif" => Self::GifChirho,
            "bmp" => Self::BmpChirho,
            "tif" | "tiff" => Self::TiffChirho,
            _ => Self::UnknownChirho,
        }
    }

    /// Get MIME type for the format.
    pub fn mime_type_chirho(&self) -> &'static str {
        match self {
            Self::JpegChirho => "image/jpeg",
            Self::PngChirho => "image/png",
            Self::GifChirho => "image/gif",
            Self::BmpChirho => "image/bmp",
            Self::TiffChirho => "image/tiff",
            Self::UnknownChirho => "application/octet-stream",
        }
    }

    /// Get common file extension.
    pub fn extension_chirho(&self) -> &'static str {
        match self {
            Self::JpegChirho => "jpg",
            Self::PngChirho => "png",
            Self::GifChirho => "gif",
            Self::BmpChirho => "bmp",
            Self::TiffChirho => "tiff",
            Self::UnknownChirho => "bin",
        }
    }
}

/// Hotspot shape for map regions.
#[derive(Debug, Clone, PartialEq)]
pub enum HotspotShapeChirho {
    /// Rectangular region.
    RectChirho {
        /// Left coordinate.
        x1_chirho: i32,
        /// Top coordinate.
        y1_chirho: i32,
        /// Right coordinate.
        x2_chirho: i32,
        /// Bottom coordinate.
        y2_chirho: i32,
    },
    /// Circular region.
    CircleChirho {
        /// Center X coordinate.
        cx_chirho: i32,
        /// Center Y coordinate.
        cy_chirho: i32,
        /// Radius.
        radius_chirho: i32,
    },
    /// Polygon region.
    PolyChirho {
        /// List of (x, y) points.
        points_chirho: Vec<(i32, i32)>,
    },
}

impl HotspotShapeChirho {
    /// Check if a point is within this shape.
    pub fn contains_chirho(&self, x_chirho: i32, y_chirho: i32) -> bool {
        match self {
            Self::RectChirho { x1_chirho, y1_chirho, x2_chirho, y2_chirho } => {
                x_chirho >= *x1_chirho && x_chirho <= *x2_chirho &&
                y_chirho >= *y1_chirho && y_chirho <= *y2_chirho
            }
            Self::CircleChirho { cx_chirho, cy_chirho, radius_chirho } => {
                let dx_chirho = x_chirho - cx_chirho;
                let dy_chirho = y_chirho - cy_chirho;
                (dx_chirho * dx_chirho + dy_chirho * dy_chirho) <= (radius_chirho * radius_chirho)
            }
            Self::PolyChirho { points_chirho } => {
                // Point-in-polygon using ray casting
                if points_chirho.len() < 3 {
                    return false;
                }

                let mut inside_chirho = false;
                let n_chirho = points_chirho.len();

                let mut j_chirho = n_chirho - 1;
                for i_chirho in 0..n_chirho {
                    let (xi_chirho, yi_chirho) = points_chirho[i_chirho];
                    let (xj_chirho, yj_chirho) = points_chirho[j_chirho];

                    if ((yi_chirho > y_chirho) != (yj_chirho > y_chirho)) &&
                       (x_chirho < (xj_chirho - xi_chirho) * (y_chirho - yi_chirho) /
                                   (yj_chirho - yi_chirho) + xi_chirho)
                    {
                        inside_chirho = !inside_chirho;
                    }
                    j_chirho = i_chirho;
                }

                inside_chirho
            }
        }
    }
}

/// A clickable hotspot region on a map.
#[derive(Debug, Clone)]
pub struct HotspotChirho {
    /// Unique identifier for this hotspot.
    pub id_chirho: String,
    /// Display name/label.
    pub name_chirho: String,
    /// Shape defining the hotspot region.
    pub shape_chirho: HotspotShapeChirho,
    /// Target URL or reference (e.g., verse reference).
    pub target_chirho: Option<String>,
    /// Optional description or tooltip text.
    pub description_chirho: Option<String>,
}

impl HotspotChirho {
    /// Create a new rectangular hotspot.
    pub fn rect_chirho(
        id_chirho: &str,
        name_chirho: &str,
        x1_chirho: i32,
        y1_chirho: i32,
        x2_chirho: i32,
        y2_chirho: i32,
    ) -> Self {
        Self {
            id_chirho: id_chirho.to_string(),
            name_chirho: name_chirho.to_string(),
            shape_chirho: HotspotShapeChirho::RectChirho {
                x1_chirho,
                y1_chirho,
                x2_chirho,
                y2_chirho,
            },
            target_chirho: None,
            description_chirho: None,
        }
    }

    /// Create a new circular hotspot.
    pub fn circle_chirho(
        id_chirho: &str,
        name_chirho: &str,
        cx_chirho: i32,
        cy_chirho: i32,
        radius_chirho: i32,
    ) -> Self {
        Self {
            id_chirho: id_chirho.to_string(),
            name_chirho: name_chirho.to_string(),
            shape_chirho: HotspotShapeChirho::CircleChirho {
                cx_chirho,
                cy_chirho,
                radius_chirho,
            },
            target_chirho: None,
            description_chirho: None,
        }
    }

    /// Set the target reference.
    pub fn with_target_chirho(mut self, target_chirho: &str) -> Self {
        self.target_chirho = Some(target_chirho.to_string());
        self
    }

    /// Set the description.
    pub fn with_description_chirho(mut self, description_chirho: &str) -> Self {
        self.description_chirho = Some(description_chirho.to_string());
        self
    }

    /// Check if coordinates hit this hotspot.
    pub fn hit_test_chirho(&self, x_chirho: i32, y_chirho: i32) -> bool {
        self.shape_chirho.contains_chirho(x_chirho, y_chirho)
    }
}

/// Map overlay containing hotspot definitions.
#[derive(Debug, Clone, Default)]
pub struct MapOverlayChirho {
    /// Image key this overlay applies to.
    pub image_key_chirho: String,
    /// Width of the image (for coordinate scaling).
    pub width_chirho: u32,
    /// Height of the image (for coordinate scaling).
    pub height_chirho: u32,
    /// List of hotspots on this map.
    pub hotspots_chirho: Vec<HotspotChirho>,
}

impl MapOverlayChirho {
    /// Create a new map overlay.
    pub fn new_chirho(image_key_chirho: &str, width_chirho: u32, height_chirho: u32) -> Self {
        Self {
            image_key_chirho: image_key_chirho.to_string(),
            width_chirho,
            height_chirho,
            hotspots_chirho: Vec::new(),
        }
    }

    /// Add a hotspot.
    pub fn add_hotspot_chirho(&mut self, hotspot_chirho: HotspotChirho) {
        self.hotspots_chirho.push(hotspot_chirho);
    }

    /// Find hotspot at coordinates.
    pub fn hit_test_chirho(&self, x_chirho: i32, y_chirho: i32) -> Option<&HotspotChirho> {
        self.hotspots_chirho.iter().find(|h_chirho| h_chirho.hit_test_chirho(x_chirho, y_chirho))
    }

    /// Find all hotspots at coordinates (for overlapping regions).
    pub fn hit_test_all_chirho(&self, x_chirho: i32, y_chirho: i32) -> Vec<&HotspotChirho> {
        self.hotspots_chirho
            .iter()
            .filter(|h_chirho| h_chirho.hit_test_chirho(x_chirho, y_chirho))
            .collect()
    }

    /// Parse hotspot definitions from a configuration string.
    ///
    /// Format: SHAPE;ID;NAME;COORDS;TARGET
    /// - RECT;id;name;x1,y1,x2,y2;target
    /// - CIRCLE;id;name;cx,cy,r;target
    /// - POLY;id;name;x1,y1,x2,y2,...;target
    pub fn parse_hotspots_chirho(&mut self, config_chirho: &str) -> ResultChirho<()> {
        for line_chirho in config_chirho.lines() {
            let line_chirho = line_chirho.trim();
            if line_chirho.is_empty() || line_chirho.starts_with('#') {
                continue;
            }

            let parts_chirho: Vec<&str> = line_chirho.split(';').collect();
            if parts_chirho.len() < 4 {
                continue; // Invalid line
            }

            let shape_type_chirho = parts_chirho[0].to_uppercase();
            let id_chirho = parts_chirho[1];
            let name_chirho = parts_chirho[2];
            let coords_chirho = parts_chirho[3];
            let target_chirho = parts_chirho.get(4).copied();

            let hotspot_chirho = match shape_type_chirho.as_str() {
                "RECT" => {
                    let coords_chirho: Vec<i32> = coords_chirho
                        .split(',')
                        .filter_map(|s_chirho| s_chirho.trim().parse().ok())
                        .collect();

                    if coords_chirho.len() != 4 {
                        continue;
                    }

                    let mut h_chirho = HotspotChirho::rect_chirho(
                        id_chirho,
                        name_chirho,
                        coords_chirho[0],
                        coords_chirho[1],
                        coords_chirho[2],
                        coords_chirho[3],
                    );

                    if let Some(t_chirho) = target_chirho {
                        h_chirho = h_chirho.with_target_chirho(t_chirho);
                    }

                    h_chirho
                }
                "CIRCLE" => {
                    let coords_chirho: Vec<i32> = coords_chirho
                        .split(',')
                        .filter_map(|s_chirho| s_chirho.trim().parse().ok())
                        .collect();

                    if coords_chirho.len() != 3 {
                        continue;
                    }

                    let mut h_chirho = HotspotChirho::circle_chirho(
                        id_chirho,
                        name_chirho,
                        coords_chirho[0],
                        coords_chirho[1],
                        coords_chirho[2],
                    );

                    if let Some(t_chirho) = target_chirho {
                        h_chirho = h_chirho.with_target_chirho(t_chirho);
                    }

                    h_chirho
                }
                "POLY" => {
                    let nums_chirho: Vec<i32> = coords_chirho
                        .split(',')
                        .filter_map(|s_chirho| s_chirho.trim().parse().ok())
                        .collect();

                    if nums_chirho.len() < 6 || !nums_chirho.len().is_multiple_of(2) {
                        continue;
                    }

                    let points_chirho: Vec<(i32, i32)> = nums_chirho
                        .chunks(2)
                        .map(|c_chirho| (c_chirho[0], c_chirho[1]))
                        .collect();

                    let mut h_chirho = HotspotChirho {
                        id_chirho: id_chirho.to_string(),
                        name_chirho: name_chirho.to_string(),
                        shape_chirho: HotspotShapeChirho::PolyChirho { points_chirho },
                        target_chirho: None,
                        description_chirho: None,
                    };

                    if let Some(t_chirho) = target_chirho {
                        h_chirho = h_chirho.with_target_chirho(t_chirho);
                    }

                    h_chirho
                }
                _ => continue,
            };

            self.hotspots_chirho.push(hotspot_chirho);
        }

        Ok(())
    }
}

/// An image entry with binary data and metadata.
#[derive(Debug, Clone)]
pub struct ImageEntryChirho {
    /// Key/name of the image.
    pub key_chirho: String,
    /// Image format.
    pub format_chirho: ImageFormatChirho,
    /// Binary image data.
    pub data_chirho: Vec<u8>,
    /// Width in pixels (if known).
    pub width_chirho: Option<u32>,
    /// Height in pixels (if known).
    pub height_chirho: Option<u32>,
    /// Associated map overlay (if this is a map).
    pub overlay_chirho: Option<MapOverlayChirho>,
}

impl ImageEntryChirho {
    /// Create a new image entry.
    pub fn new_chirho(key_chirho: &str, format_chirho: ImageFormatChirho, data_chirho: Vec<u8>) -> Self {
        Self {
            key_chirho: key_chirho.to_string(),
            format_chirho,
            data_chirho,
            width_chirho: None,
            height_chirho: None,
            overlay_chirho: None,
        }
    }

    /// Get the MIME type.
    pub fn mime_type_chirho(&self) -> &'static str {
        self.format_chirho.mime_type_chirho()
    }

    /// Get data size in bytes.
    pub fn size_chirho(&self) -> usize {
        self.data_chirho.len()
    }

    /// Check if this image has a map overlay.
    pub fn has_overlay_chirho(&self) -> bool {
        self.overlay_chirho.is_some()
    }
}

/// Image/map module driver.
///
/// Provides access to SWORD image modules containing maps, diagrams,
/// and other visual content.
pub struct ImageModuleChirho {
    /// Module name.
    name_chirho: String,
    /// Module description.
    description_chirho: String,
    /// Base path to module data.
    data_path_chirho: PathBuf,
    /// Cached image entries.
    images_chirho: HashMap<String, ImageEntryChirho>,
    /// Map overlays keyed by image name.
    overlays_chirho: HashMap<String, MapOverlayChirho>,
}

impl ImageModuleChirho {
    /// Create a new image module.
    pub fn new_chirho(config_chirho: &ModuleConfigChirho, base_path_chirho: &Path) -> ResultChirho<Self> {
        let name_chirho = config_chirho.get_chirho("Description")
            .unwrap_or(config_chirho.name_chirho.as_str())
            .to_string();

        let description_chirho = config_chirho.get_chirho("About")
            .unwrap_or("")
            .to_string();

        let data_path_str_chirho = config_chirho.get_chirho("DataPath")
            .ok_or_else(|| ErrorChirho::invalid_config_chirho("Missing DataPath in config"))?;

        let data_path_chirho = base_path_chirho.join(data_path_str_chirho);

        Ok(Self {
            name_chirho,
            description_chirho,
            data_path_chirho,
            images_chirho: HashMap::new(),
            overlays_chirho: HashMap::new(),
        })
    }

    /// Create an image module from a path (without config).
    pub fn from_path_chirho(name_chirho: &str, path_chirho: &Path) -> ResultChirho<Self> {
        if !path_chirho.exists() {
            return Err(ErrorChirho::generic_chirho(format!(
                "Image module path does not exist: {}",
                path_chirho.display()
            )));
        }

        Ok(Self {
            name_chirho: name_chirho.to_string(),
            description_chirho: String::new(),
            data_path_chirho: path_chirho.to_path_buf(),
            images_chirho: HashMap::new(),
            overlays_chirho: HashMap::new(),
        })
    }

    /// Get the module name.
    pub fn name_chirho(&self) -> &str {
        &self.name_chirho
    }

    /// Get the module description.
    pub fn description_chirho(&self) -> &str {
        &self.description_chirho
    }

    /// Get the module type identifier.
    pub fn module_type_chirho(&self) -> &str {
        crate::module_type_chirho::IMAGES_CHIRHO
    }

    /// Load all images from the data path.
    pub fn load_all_chirho(&mut self) -> ResultChirho<()> {
        if !self.data_path_chirho.exists() {
            return Err(ErrorChirho::generic_chirho(format!(
                "Data path does not exist: {}",
                self.data_path_chirho.display()
            )));
        }

        // Load images
        self.scan_directory_chirho(&self.data_path_chirho.clone())?;

        // Load overlay files (.map, .hotspot, etc.)
        self.load_overlays_chirho()?;

        Ok(())
    }

    /// Scan directory for image files.
    fn scan_directory_chirho(&mut self, dir_chirho: &Path) -> ResultChirho<()> {
        if !dir_chirho.is_dir() {
            return Ok(());
        }

        let entries_chirho = fs::read_dir(dir_chirho)?;

        for entry_result_chirho in entries_chirho {
            let entry_chirho = entry_result_chirho?;
            let path_chirho = entry_chirho.path();

            if path_chirho.is_dir() {
                // Recurse into subdirectories
                self.scan_directory_chirho(&path_chirho)?;
            } else if let Some(ext_chirho) = path_chirho.extension() {
                let ext_str_chirho = ext_chirho.to_string_lossy().to_lowercase();
                let format_chirho = ImageFormatChirho::from_extension_chirho(&ext_str_chirho);

                if format_chirho != ImageFormatChirho::UnknownChirho {
                    // Load image
                    let data_chirho = fs::read(&path_chirho)?;

                    let key_chirho = path_chirho
                        .strip_prefix(&self.data_path_chirho)
                        .unwrap_or(&path_chirho)
                        .to_string_lossy()
                        .to_string();

                    let entry_chirho = ImageEntryChirho::new_chirho(&key_chirho, format_chirho, data_chirho);
                    self.images_chirho.insert(key_chirho, entry_chirho);
                }
            }
        }

        Ok(())
    }

    /// Load overlay/hotspot definition files.
    fn load_overlays_chirho(&mut self) -> ResultChirho<()> {
        // Look for .map or .hotspot files alongside images
        let keys_chirho: Vec<String> = self.images_chirho.keys().cloned().collect();

        for key_chirho in keys_chirho {
            let base_path_chirho = self.data_path_chirho.join(&key_chirho);

            // Try various overlay file extensions
            for ext_chirho in &["map", "hotspot", "overlay"] {
                let overlay_path_chirho = base_path_chirho.with_extension(ext_chirho);

                if overlay_path_chirho.exists() {
                    let content_chirho = fs::read_to_string(&overlay_path_chirho)?;

                    let mut overlay_chirho = MapOverlayChirho::new_chirho(&key_chirho, 0, 0);
                    overlay_chirho.parse_hotspots_chirho(&content_chirho)?;

                    self.overlays_chirho.insert(key_chirho.clone(), overlay_chirho);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Get an image by key.
    pub fn get_image_chirho(&self, key_chirho: &str) -> Option<&ImageEntryChirho> {
        self.images_chirho.get(key_chirho)
    }

    /// Get raw image data by key.
    pub fn get_data_chirho(&self, key_chirho: &str) -> Option<&[u8]> {
        self.images_chirho.get(key_chirho).map(|e_chirho| e_chirho.data_chirho.as_slice())
    }

    /// Get map overlay for an image.
    pub fn get_overlay_chirho(&self, key_chirho: &str) -> Option<&MapOverlayChirho> {
        self.overlays_chirho.get(key_chirho)
    }

    /// List all image keys.
    pub fn list_keys_chirho(&self) -> Vec<&str> {
        self.images_chirho.keys().map(|k_chirho| k_chirho.as_str()).collect()
    }

    /// Get the count of images.
    pub fn count_chirho(&self) -> usize {
        self.images_chirho.len()
    }

    /// Check if a key exists.
    pub fn has_key_chirho(&self, key_chirho: &str) -> bool {
        self.images_chirho.contains_key(key_chirho)
    }

    /// Hit test a point on a map image.
    pub fn hit_test_chirho(&self, key_chirho: &str, x_chirho: i32, y_chirho: i32) -> Option<&HotspotChirho> {
        self.overlays_chirho.get(key_chirho)?.hit_test_chirho(x_chirho, y_chirho)
    }

    /// Render text representation of an image entry.
    pub fn render_text_chirho(&self, key_chirho: &str) -> String {
        if let Some(entry_chirho) = self.get_image_chirho(key_chirho) {
            format!(
                "[Image: {} ({}, {} bytes)]",
                key_chirho,
                entry_chirho.format_chirho.mime_type_chirho(),
                entry_chirho.size_chirho()
            )
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_image_format_from_extension_chirho() {
        assert_eq!(ImageFormatChirho::from_extension_chirho("jpg"), ImageFormatChirho::JpegChirho);
        assert_eq!(ImageFormatChirho::from_extension_chirho("JPEG"), ImageFormatChirho::JpegChirho);
        assert_eq!(ImageFormatChirho::from_extension_chirho("png"), ImageFormatChirho::PngChirho);
        assert_eq!(ImageFormatChirho::from_extension_chirho("gif"), ImageFormatChirho::GifChirho);
        assert_eq!(ImageFormatChirho::from_extension_chirho("bmp"), ImageFormatChirho::BmpChirho);
        assert_eq!(ImageFormatChirho::from_extension_chirho("xyz"), ImageFormatChirho::UnknownChirho);
    }

    #[test]
    fn test_image_format_mime_type_chirho() {
        assert_eq!(ImageFormatChirho::JpegChirho.mime_type_chirho(), "image/jpeg");
        assert_eq!(ImageFormatChirho::PngChirho.mime_type_chirho(), "image/png");
    }

    #[test]
    fn test_hotspot_rect_hit_test_chirho() {
        let hotspot_chirho = HotspotChirho::rect_chirho("test", "Test", 10, 10, 100, 100);

        assert!(hotspot_chirho.hit_test_chirho(50, 50));
        assert!(hotspot_chirho.hit_test_chirho(10, 10));
        assert!(hotspot_chirho.hit_test_chirho(100, 100));
        assert!(!hotspot_chirho.hit_test_chirho(5, 50));
        assert!(!hotspot_chirho.hit_test_chirho(105, 50));
    }

    #[test]
    fn test_hotspot_circle_hit_test_chirho() {
        let hotspot_chirho = HotspotChirho::circle_chirho("test", "Test", 50, 50, 25);

        assert!(hotspot_chirho.hit_test_chirho(50, 50)); // Center
        assert!(hotspot_chirho.hit_test_chirho(50, 25)); // Top edge
        assert!(hotspot_chirho.hit_test_chirho(75, 50)); // Right edge
        assert!(!hotspot_chirho.hit_test_chirho(10, 10)); // Outside
        assert!(!hotspot_chirho.hit_test_chirho(80, 80)); // Outside
    }

    #[test]
    fn test_hotspot_with_target_chirho() {
        let hotspot_chirho = HotspotChirho::rect_chirho("jerusalem", "Jerusalem", 100, 100, 150, 150)
            .with_target_chirho("Gen.14.18")
            .with_description_chirho("City of Peace");

        assert_eq!(hotspot_chirho.target_chirho, Some("Gen.14.18".to_string()));
        assert_eq!(hotspot_chirho.description_chirho, Some("City of Peace".to_string()));
    }

    #[test]
    fn test_map_overlay_hit_test_chirho() {
        let mut overlay_chirho = MapOverlayChirho::new_chirho("map.jpg", 800, 600);

        overlay_chirho.add_hotspot_chirho(
            HotspotChirho::rect_chirho("region1", "Region 1", 0, 0, 100, 100)
        );
        overlay_chirho.add_hotspot_chirho(
            HotspotChirho::rect_chirho("region2", "Region 2", 200, 200, 300, 300)
        );

        let hit_chirho = overlay_chirho.hit_test_chirho(50, 50);
        assert!(hit_chirho.is_some());
        assert_eq!(hit_chirho.unwrap().id_chirho, "region1");

        let hit_chirho = overlay_chirho.hit_test_chirho(250, 250);
        assert!(hit_chirho.is_some());
        assert_eq!(hit_chirho.unwrap().id_chirho, "region2");

        let miss_chirho = overlay_chirho.hit_test_chirho(150, 150);
        assert!(miss_chirho.is_none());
    }

    #[test]
    fn test_parse_hotspots_chirho() {
        let mut overlay_chirho = MapOverlayChirho::new_chirho("test.jpg", 800, 600);

        let config_chirho = r#"
# This is a comment
RECT;city1;Jerusalem;100,100,150,150;Gen.14.18
CIRCLE;region1;Galilee;300,200,50;Matt.4.18
POLY;area1;Judea;10,10,50,10,50,50,10,50;Luke.1.5
"#;

        overlay_chirho.parse_hotspots_chirho(config_chirho).unwrap();

        assert_eq!(overlay_chirho.hotspots_chirho.len(), 3);

        // Check rect
        let h1_chirho = &overlay_chirho.hotspots_chirho[0];
        assert_eq!(h1_chirho.id_chirho, "city1");
        assert_eq!(h1_chirho.name_chirho, "Jerusalem");
        assert_eq!(h1_chirho.target_chirho, Some("Gen.14.18".to_string()));

        // Check circle
        let h2_chirho = &overlay_chirho.hotspots_chirho[1];
        assert_eq!(h2_chirho.id_chirho, "region1");
        assert!(matches!(h2_chirho.shape_chirho, HotspotShapeChirho::CircleChirho { .. }));

        // Check poly
        let h3_chirho = &overlay_chirho.hotspots_chirho[2];
        assert_eq!(h3_chirho.id_chirho, "area1");
        assert!(matches!(h3_chirho.shape_chirho, HotspotShapeChirho::PolyChirho { .. }));
    }

    #[test]
    fn test_image_entry_chirho() {
        let data_chirho = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG magic bytes
        let entry_chirho = ImageEntryChirho::new_chirho("test.jpg", ImageFormatChirho::JpegChirho, data_chirho);

        assert_eq!(entry_chirho.key_chirho, "test.jpg");
        assert_eq!(entry_chirho.mime_type_chirho(), "image/jpeg");
        assert_eq!(entry_chirho.size_chirho(), 4);
        assert!(!entry_chirho.has_overlay_chirho());
    }

    #[test]
    fn test_polygon_hit_test_chirho() {
        // Simple square polygon
        let shape_chirho = HotspotShapeChirho::PolyChirho {
            points_chirho: vec![(0, 0), (100, 0), (100, 100), (0, 100)],
        };

        assert!(shape_chirho.contains_chirho(50, 50));
        assert!(shape_chirho.contains_chirho(1, 1));
        assert!(shape_chirho.contains_chirho(99, 99));
        assert!(!shape_chirho.contains_chirho(150, 50));
        assert!(!shape_chirho.contains_chirho(-10, 50));
    }

    #[test]
    fn test_image_module_from_path_chirho() {
        use tempfile::tempdir;

        let dir_chirho = tempdir().unwrap();
        let mut module_chirho = ImageModuleChirho::from_path_chirho("TestImages", dir_chirho.path()).unwrap();

        assert_eq!(module_chirho.name_chirho(), "TestImages");
        assert_eq!(module_chirho.module_type_chirho(), crate::module_type_chirho::IMAGES_CHIRHO);

        // Load (empty directory)
        module_chirho.load_all_chirho().unwrap();
        assert_eq!(module_chirho.count_chirho(), 0);
    }
}
