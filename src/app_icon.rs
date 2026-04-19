use iced::window::icon;
use iced::window::Icon;

const ICON_SIZE: u32 = 512;
const SVG_VIEWBOX: f32 = 690.0;
const BARS: [(f32, f32, f32, f32, f32); 5] = [
    (198.0, 390.0, 52.0, 108.0, 26.0),
    (266.0, 336.0, 52.0, 162.0, 26.0),
    (334.0, 274.0, 52.0, 224.0, 26.0),
    (402.0, 204.0, 52.0, 294.0, 26.0),
    (470.0, 124.0, 52.0, 374.0, 26.0),
];
const BACKGROUND: (u8, u8, u8) = (17, 17, 17);
const ACCENT: (u8, u8, u8) = (249, 115, 22);

pub fn window_icon() -> Icon {
    icon::from_rgba(render_logo_rgba(ICON_SIZE), ICON_SIZE, ICON_SIZE)
        .expect("generated app window icon should be valid")
}

fn render_logo_rgba(size: u32) -> Vec<u8> {
    let samples_per_axis = 4;
    let total_samples = (samples_per_axis * samples_per_axis) as f32;
    let scale = SVG_VIEWBOX / size as f32;

    let mut pixels = vec![0u8; (size * size * 4) as usize];

    for y in 0..size {
        for x in 0..size {
            let mut covered_samples = 0.0;

            for sample_y in 0..samples_per_axis {
                for sample_x in 0..samples_per_axis {
                    let svg_x = (x as f32 + (sample_x as f32 + 0.5) / samples_per_axis as f32)
                        * scale;
                    let svg_y = (y as f32 + (sample_y as f32 + 0.5) / samples_per_axis as f32)
                        * scale;

                    if point_in_bars(svg_x, svg_y) {
                        covered_samples += 1.0;
                    }
                }
            }

            if covered_samples > 0.0 {
                let index = ((y * size + x) * 4) as usize;
                let inverse_coverage = 1.0 / covered_samples;

                pixels[index] = ACCENT.0;
                pixels[index + 1] = ACCENT.1;
                pixels[index + 2] = ACCENT.2;
                pixels[index + 3] = ((covered_samples / total_samples) * 255.0).round() as u8;
            }
        }
    }

    pixels
}

fn point_in_bars(px: f32, py: f32) -> bool {
    BARS.iter().any(|bar| point_in_rounded_rect(px, py, *bar))
}

fn point_in_rounded_rect(px: f32, py: f32, rect: (f32, f32, f32, f32, f32)) -> bool {
    let (x, y, width, height, radius) = rect;

    if px < x || px > x + width || py < y || py > y + height {
        return false;
    }

    let inner_left = x + radius;
    let inner_right = x + width - radius;
    let inner_top = y + radius;
    let inner_bottom = y + height - radius;

    if (px >= inner_left && px <= inner_right) || (py >= inner_top && py <= inner_bottom) {
        return true;
    }

    let nearest_x = if px < inner_left { inner_left } else { inner_right };
    let nearest_y = if py < inner_top { inner_top } else { inner_bottom };

    let dx = px - nearest_x;
    let dy = py - nearest_y;

    dx * dx + dy * dy <= radius * radius
}