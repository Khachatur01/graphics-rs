// impl<Id> Render for SelectTool<Id> {
//     fn render(&self, renderer: &mut dyn Renderer) {
//         let Some(selection) = self.selection else {
//             return;
//         };
//
//         let selection_color: Color =
//             if selection.width() < 0.0 {
//                 Color(0x00, 0x80, 0x00, 0xFF)
//             } else {
//                 Color(0x00, 0x00, 0xFF, 0xFF)
//             };
//
//         renderer.rectangle(&selection, &ShapeStyle {
//             fill_color: Color::transparent(),
//             stroke_color: selection_color,
//             stroke_width: 1.0,
//             stroke_dash_array: vec![4, 4],
//         });
//     }
// }
