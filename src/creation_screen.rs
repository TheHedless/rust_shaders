pub(crate) mod creation_screen_mod {
    use std::fmt::Debug;
    use eframe::emath;
    use serde::{Deserialize, Serialize};
    use eframe::emath::{Pos2, Vec2};
    use eframe::epaint::{Rect, Shape, Stroke, PathShape};
    use egui::{Color32, Grid, Sense};
    use egui::text_selection::text_cursor_state::cursor_rect;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreationScreen {
        pub _id: String,
        pub stroke: Stroke,
        pub node: Vec<Pos2>,
        pub clicked_node:Option<Pos2>
    }
    impl Default for CreationScreen {
        fn default() -> Self {
            Self {
                stroke: Stroke::new(1.0, Color32::from_rgb(100, 100, 100)),
                node: Vec::from([
                    Pos2::new(100.0, 100.0),
                    Pos2::new(100.0, 200.0),
                    Pos2::new(50.0, 150.0)]),
                _id: "".to_string(),
                clicked_node: None,
            }
        }
    }
    impl Clone for CreationScreen {
        fn clone(&self) -> Self {
            CreationScreen {
                stroke: self.stroke,
                node: self.node.clone(),
                _id: self._id.clone(),
                clicked_node: self.clicked_node.clone();
            }
        }
    }
    impl CreationScreen {
        pub(crate) fn ui_controls(&mut self, ui: &mut egui::Ui) {}

        pub(crate) fn ui_canvas(&mut self, ui: &mut egui::Ui) {
            // define our canvas
            let (response, painter) =
                ui.allocate_painter(Vec2::new(ui.available_width(), ui.available_height()), Sense::hover());
            // normalise coords to canvas instead of the window
            let to_screen = emath::RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                response.rect,
            );
            let node_centers: Vec<Pos2> = self
                .node
                .iter()
                .map(|point| {
                    to_screen.transform_pos(to_screen.from().clamp(*point))
                })
                .collect();
            // draggable circles
            // using node_centers would save us point_in_screen having to be calculated twice
            // however drag logic breaks if it is used
            for (i, node) in self.node.iter_mut().enumerate() {
                let size = Vec2::splat(8.0);
                //gets the points as uniques
                let point_in_screen = to_screen.transform_pos(*node);
                let point_rect = Rect::from_center_size(point_in_screen, size);
                let point_id = response.id.with(i);
                //drag logic
                let point_response = ui.interact(point_rect, point_id, Sense::drag());
                *node = to_screen.from().clamp(*node + point_response.drag_delta());
            }
            let node_circles: Vec<Shape> = self
                .node
                .iter()
                .map(|point| {
                    Shape::circle_stroke(
                        to_screen.transform_pos(*point),
                        4.0,
                        Stroke::new(1.0, Color32::from_rgb(255, 255, 255)))
                })
                .collect();
            painter.extend(node_circles); // interact_pos is the one you want to anchor it
            Shape::line(
                vec![self.node[0],],
                Stroke::default()
            );
        }
    }
}