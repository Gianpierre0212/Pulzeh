use eframe::egui;

/// Dibuja una barra de progreso con el valor y color recibidos.
pub fn barra_progreso(ui: &mut egui::Ui, valor: f32, color: egui::Color32) {
    ui.add(
        egui::ProgressBar::new(valor.clamp(0.0, 1.0))
            .fill(color)
            .show_percentage(),
    );
}
pub fn estado_recurso(porcentaje: f32) -> &'static str {
    if porcentaje < 60.0 {
        "Normal"
    } else if porcentaje < 85.0 {
        "Atención"
    } else {
        "Alto"
    }
}
pub fn color_estado(porcentaje: f32) -> egui::Color32 {
    if porcentaje < 60.0 {
        egui::Color32::from_rgb(66, 217, 208)
    } else if porcentaje < 85.0 {
        egui::Color32::from_rgb(235, 190, 70)
    } else {
        egui::Color32::from_rgb(220, 80, 80)
    }
}
