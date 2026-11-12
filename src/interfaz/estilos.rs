use eframe::egui;

/// Aplica la apariencia visual principal de PULZEH.
pub fn aplicar(contexto: &egui::Context) {
    let mut visuales = egui::Visuals::dark();

    // Fondo principal: #101318.
    visuales.panel_fill = egui::Color32::from_rgb(16, 19, 24);

    // Fondo de ventanas y tarjetas: #191F27.
    visuales.window_fill = egui::Color32::from_rgb(25, 31, 39);

    // Color principal del texto: #E7EDF2.
    visuales.override_text_color = Some(egui::Color32::from_rgb(231, 237, 242));

    contexto.set_visuals(visuales);
}
