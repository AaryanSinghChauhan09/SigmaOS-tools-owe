//! SigmaOS Tools Owe - Hyprland live wallpaper engine.

use std::path::{Path, PathBuf};

/// Wallpaper media type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperKind {
    Video,
    Gif,
    Still,
    Unsupported,
}

/// Active wallpaper descriptor.
#[derive(Debug, Clone)]
pub struct WallpaperDescriptor {
    path: PathBuf,
    kind: WallpaperKind,
    output: String,
    loop_video: bool,
}

impl WallpaperDescriptor {
    pub fn new<P: AsRef<Path>>(path: P, output: &str) -> Self {
        let p = path.as_ref().to_path_buf();
        let kind = match p
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase()
            .as_str()
        {
            "mp4" | "mkv" | "webm" | "mov" | "avi" | "ogv" => WallpaperKind::Video,
            "gif" => WallpaperKind::Gif,
            "png" | "jpg" | "jpeg" | "webp" => WallpaperKind::Still,
            _ => WallpaperKind::Unsupported,
        };

        Self {
            path: p,
            kind,
            output: output.to_string(),
            loop_video: true,
        }
    }

    pub fn kind(&self) -> WallpaperKind {
        self.kind
    }

    pub fn is_playable(&self) -> bool {
        self.kind != WallpaperKind::Unsupported
    }

    pub fn output(&self) -> &str {
        &self.output
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Wallpaper manager daemon.
#[derive(Debug, Clone, Default)]
pub struct OweEngine {
    active_wallpapers: Vec<WallpaperDescriptor>,
}

impl OweEngine {
    pub fn new() -> Self {
        Self {
            active_wallpapers: Vec::new(),
        }
    }

    pub fn set_wallpaper(&mut self, desc: WallpaperDescriptor) -> Result<(), &'static str> {
        if !desc.is_playable() {
            return Err("Unsupported media format");
        }
        self.active_wallpapers.retain(|w| w.output != desc.output);
        self.active_wallpapers.push(desc);
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.active_wallpapers.len()
    }

    pub fn get_for_output(&self, output: &str) -> Option<&WallpaperDescriptor> {
        self.active_wallpapers.iter().find(|w| w.output == output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallpaper_detection_aaa() {
        // Arrange & Act
        let vid = WallpaperDescriptor::new("/usr/share/wallpapers/matrix.mp4", "eDP-1");
        let gif = WallpaperDescriptor::new("/usr/share/wallpapers/anime.gif", "HDMI-A-1");
        let still = WallpaperDescriptor::new("/usr/share/wallpapers/mountains.png", "DP-1");
        let invalid = WallpaperDescriptor::new("/usr/share/wallpapers/doc.pdf", "eDP-1");

        // Assert
        assert_eq!(vid.kind(), WallpaperKind::Video);
        assert_eq!(gif.kind(), WallpaperKind::Gif);
        assert_eq!(still.kind(), WallpaperKind::Still);
        assert_eq!(invalid.kind(), WallpaperKind::Unsupported);
        assert!(!invalid.is_playable());
    }

    #[test]
    fn engine_dispatch_aaa() {
        // Arrange
        let mut engine = OweEngine::new();
        let vid = WallpaperDescriptor::new("/wallpapers/neon.mp4", "eDP-1");

        // Act
        engine
            .set_wallpaper(vid)
            .expect("Valid wallpaper must be set");

        // Assert
        assert_eq!(engine.count(), 1);
        let current = engine.get_for_output("eDP-1").unwrap();
        assert_eq!(current.kind(), WallpaperKind::Video);
    }
}
