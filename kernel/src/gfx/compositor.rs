use super::framebuffer::{Color, Framebuffer};
use super::brain::NeuralBrainMesh;
use crate::{println, serial_println};

pub struct WorkspaceCompositor {
    brain_mesh: NeuralBrainMesh,
}

impl WorkspaceCompositor {
    pub fn new() -> Self {
        WorkspaceCompositor {
            brain_mesh: NeuralBrainMesh::new(),
        }
    }

    pub fn render_workspace(&self, fb: &mut Framebuffer) {
        println!("[COMPOSITOR] Rendering Graphical 3D Brain Workspace Shell...");
        serial_println!("[COMPOSITOR] Rendering Graphical 3D Brain Workspace Shell...");

        // 1. Clear Screen to Dark Space Background
        fb.clear(Color::DARK_SPACE);

        // 2. Render Top System Status Bar (Height 32px)
        fb.draw_rect(0, 0, fb.width, 32, Color::BLACK);
        fb.draw_line(0, 32, fb.width, 32, Color::CYAN);

        // Top Status Bar Accent Elements
        fb.draw_rect(10, 8, 16, 16, Color::CYAN); // Zenith Core Logo Icon
        if fb.width > 120 {
            fb.draw_rect(fb.width - 110, 10, 100, 12, Color::ACCENT_GREEN); // Heap Gauge
        }

        // 3. Render 3D Neural Brain Core in Center
        let center_x = fb.width / 2;
        let center_y = fb.height / 2;
        self.brain_mesh.render(fb, center_x, center_y);

        // 4. Render Bottom Application Dock (Height 44px)
        let dock_width = if fb.width > 240 { 240 } else { fb.width.saturating_sub(20) };
        let dock_x = (fb.width.saturating_sub(dock_width)) / 2;
        let dock_y = fb.height.saturating_sub(50);

        fb.draw_rect(dock_x, dock_y, dock_width, 40, Color::BLACK);
        fb.draw_line(dock_x, dock_y, dock_x + dock_width, dock_y, Color::NEURAL_PURPLE);

        // Render Dock Tiles (4 Application Icons)
        for i in 0..4 {
            let tile_x = dock_x + 10 + i * 55;
            let tile_y = dock_y + 8;
            if tile_x + 24 <= fb.width && tile_y + 24 <= fb.height {
                let color = match i {
                    0 => Color::CYAN,
                    1 => Color::NEURAL_PURPLE,
                    2 => Color::ELECTRIC_BLUE,
                    _ => Color::ACCENT_GREEN,
                };
                fb.draw_rect(tile_x, tile_y, 24, 24, color);
            }
        }

        println!("[COMPOSITOR] 3D Brain Workspace Shell Rendered Successfully!");
        serial_println!("[COMPOSITOR] 3D Brain Workspace Shell Rendered Successfully!");
    }
}
