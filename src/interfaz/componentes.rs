use eframe::egui;

/// Dibuja una barra de progreso con el valor y color recibidos.
pub fn barra_progreso(ui: &mut egui::Ui, valor: f32, color: egui::Color32) {
    ui.add(
        egui::ProgressBar::new(valor.clamp(0.0, 1.0))
            .fill(color)
            .show_percentage(),
    );
}
