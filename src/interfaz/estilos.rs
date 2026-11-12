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

/// Devuelve el color de acento principal de PULZEH.
pub fn color_acento() -> egui::Color32 {
    // Color definido por la identidad visual: #42D9D0.
    egui::Color32::from_rgb(66, 217, 208)
}

/// Devuelve el color utilizado para representar la memoria RAM.
pub fn color_memoria() -> egui::Color32 {
    // Azul diferenciador para separar visualmente la RAM de la CPU.
    egui::Color32::from_rgb(76, 145, 211)
}

/// Devuelve el color secundario utilizado para textos informativos.
pub fn color_secundario() -> egui::Color32 {
    // Color definido por la identidad visual: #8995A3.
    egui::Color32::from_rgb(137, 149, 163)
}
