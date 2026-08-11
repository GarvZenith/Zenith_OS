use super::framebuffer::{Color, Framebuffer};
use alloc::vec::Vec;
use crate::{println, serial_println};

#[derive(Debug, Clone, Copy)]
pub struct NeuralNode3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub color: Color,
}

pub struct NeuralBrainMesh {
    nodes: Vec<NeuralNode3D>,
    edges: Vec<(usize, usize)>,
}

impl NeuralBrainMesh {
    pub fn new() -> Self {
        let mut nodes = Vec::new();
        // Generate 3D Neural Nodes layout (Brain hemisphere concept)
        nodes.push(NeuralNode3D { x: 0, y: -40, z: 0, color: Color::CYAN }); // Core Node
        nodes.push(NeuralNode3D { x: -60, y: -20, z: 30, color: Color::NEURAL_PURPLE }); // Frontal Lobe
        nodes.push(NeuralNode3D { x: 60, y: -20, z: 30, color: Color::NEURAL_PURPLE }); // Parietal Lobe
        nodes.push(NeuralNode3D { x: -80, y: 30, z: -10, color: Color::ELECTRIC_BLUE }); // Temporal Left
        nodes.push(NeuralNode3D { x: 80, y: 30, z: -10, color: Color::ELECTRIC_BLUE }); // Temporal Right
        nodes.push(NeuralNode3D { x: 0, y: 60, z: -40, color: Color::ACCENT_GREEN }); // Occipital Core
        nodes.push(NeuralNode3D { x: -30, y: 10, z: 50, color: Color::CYAN });
        nodes.push(NeuralNode3D { x: 30, y: 10, z: 50, color: Color::CYAN });

        let mut edges = Vec::new();
        edges.push((0, 1));
        edges.push((0, 2));
        edges.push((1, 3));
        edges.push((2, 4));
        edges.push((3, 5));
        edges.push((4, 5));
        edges.push((0, 6));
        edges.push((0, 7));
        edges.push((6, 1));
        edges.push((7, 2));

        NeuralBrainMesh { nodes, edges }
    }

    pub fn render(&self, fb: &mut Framebuffer, center_x: usize, center_y: usize) {
        println!("[3D BRAIN RENDERER] Projecting 3D Neural Mesh ({} Nodes, {} Synapses)...", self.nodes.len(), self.edges.len());
        serial_println!("[3D BRAIN RENDERER] Projecting 3D Neural Mesh ({} Nodes, {} Synapses)...", self.nodes.len(), self.edges.len());

        // Draw Synapse Edges
        for &(start_idx, end_idx) in &self.edges {
            let n1 = self.nodes[start_idx];
            let n2 = self.nodes[end_idx];

            let screen1_x = (center_x as i32 + n1.x) as usize;
            let screen1_y = (center_y as i32 + n1.y) as usize;
            let screen2_x = (center_x as i32 + n2.x) as usize;
            let screen2_y = (center_y as i32 + n2.y) as usize;

            fb.draw_line(screen1_x, screen1_y, screen2_x, screen2_y, Color::ELECTRIC_BLUE);
        }

        // Draw Neural Nodes
        for node in &self.nodes {
            let screen_x = (center_x as i32 + node.x) as usize;
            let screen_y = (center_y as i32 + node.y) as usize;

            if screen_x >= 4 && screen_y >= 4 {
                fb.draw_rect(screen_x - 3, screen_y - 3, 7, 7, node.color);
            }
        }
    }
}
