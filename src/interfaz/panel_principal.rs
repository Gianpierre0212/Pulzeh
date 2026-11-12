use eframe::egui;

/// Dibuja el panel principal de PULZEH.
pub fn mostrar(
    contexto: &egui::Context,
    cpu_sin_datos: bool,
    uso_cpu_actual: f32,
    promedio_cpu: f32,
    porcentaje_memoria: f32,
    memoria_usada_gb: f32,
    memoria_total_gb: f32,
) {
    egui::CentralPanel::default().show(contexto, |ui| {
        ui.heading("PULZEH");
        ui.label("MONITOR DE RENDIMIENTO DEL SISTEMA");
        ui.add_space(20.0);

        ui.group(|ui| {
            ui.heading("CPU");

            if cpu_sin_datos {
                ui.label("Esperando primera medición...");
            } else {
                ui.label(format!("Uso actual: {uso_cpu_actual:.1}%"));
                ui.label(format!("Promedio reciente: {promedio_cpu:.1}%"));

                // La barra representa el uso instantáneo actual.
                ui.add(
                    egui::ProgressBar::new(uso_cpu_actual / 100.0)
                        .fill(super::estilos::color_acento())
                        .show_percentage(),
                );
            }
        });

        ui.add_space(12.0);

        ui.group(|ui| {
            ui.heading("Memoria RAM");
            ui.label(format!(
                "Uso actual: {porcentaje_memoria:.1}% ({memoria_usada_gb:.2} / {memoria_total_gb:.2} GB)"
            ));
            ui.add(
                egui::ProgressBar::new(porcentaje_memoria / 100.0)
                    .fill(super::estilos::color_memoria())
                    .show_percentage(),
            );
        });
    });
}
