use super::*;

#[derive(Component, Clone, Copy, Default, Debug, Inspectable)]
pub struct GridTarget {
    pub mouse: Vec2,
    pub target: Vec2,
}

#[derive(Component)]
pub struct GridTargetHex;

#[derive(Component, Clone, Copy, Default, Debug, Inspectable)]
pub struct GridMovement {
    pub cost: u32,
    pub speed: u32,
    pub distance: u32,
}

impl GridTarget {
    pub fn set_current(&mut self) {
        self.target = self.mouse;
    }

    pub fn update_current(
        &mut self,
        window: &Window,
        camera: &Camera,
        transform: &GlobalTransform,
    ) {
        if let Some(screen_pos) = window.cursor_position() {
            let node = HexNode::new(
                Vec2 {
                    x: TILE_SIZE,
                    y: TILE_SIZE,
                },
                orient::Style::Pointy,
                Vec2 { x: 0., y: 0. },
                12,
            );

            // Convert window position to gpu coordinates
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let ndc_to_world = transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            let mouse_pos = Vec2 {
                x: world_pos.x,
                y: world_pos.y,
            };

            let hex = node.layout.hex_for(mouse_pos);
            self.mouse = node.layout.center_for(&hex);
        }
    }
}

// impl GridMovement {
//     pub fn update_current(&mut self, targets: &GridTarget, trasform: &Transform) {
//         // TODO Implemt Movement target on grid line to active node
//         // The goal is to update a line from transform to target
//     }
// }
