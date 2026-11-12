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
        ui.label(
            egui::RichText::new("MONITOR DE RENDIMIENTO DEL SISTEMA")
                .color(super::estilos::color_secundario()),
        );
        ui.label(
            egui::RichText::new("Actualización automática cada segundo")
                .small()
                .color(super::estilos::color_secundario()),
        );
        ui.separator();
        ui.add_space(20.0);

        ui.columns(2, |columnas| {
            columnas[0].group(|ui| {
                ui.heading(
                    egui::RichText::new("CPU").color(super::estilos::color_acento()),
                );

                if cpu_sin_datos {
                    ui.label("Esperando primera medición...");
                } else {
                    ui.label(format!("Uso actual: {uso_cpu_actual:.1}%"));
                    ui.label(format!("Promedio reciente: {promedio_cpu:.1}%"));
                    let estado_cpu = super::componentes::estado_recurso(uso_cpu_actual);
let color_cpu = super::componentes::color_estado(uso_cpu_actual);

ui.label(
    egui::RichText::new(format!("Estado: {estado_cpu}"))
        .color(color_cpu),
);

                    // La barra representa el uso instantáneo actual.
                   super::componentes::barra_progreso(
    ui,
    uso_cpu_actual / 100.0,
    color_cpu,
);
                }
            });

            columnas[1].group(|ui| {
                ui.heading(
                    egui::RichText::new("Memoria RAM")
                        .color(super::estilos::color_memoria()),
                );

                ui.label(format!(
                    "Uso actual: {porcentaje_memoria:.1}% ({memoria_usada_gb:.2} / {memoria_total_gb:.2} GB)"
                ));

                let estado_memoria =
                    super::componentes::estado_recurso(porcentaje_memoria);
                let color_memoria =
                    super::componentes::color_estado(porcentaje_memoria);

                ui.label(
                    egui::RichText::new(format!("Estado: {estado_memoria}"))
                        .color(color_memoria),
                );

                super::componentes::barra_progreso(
                    ui,
                    porcentaje_memoria / 100.0,
                    color_memoria,
                );
            });
        });
    });
}
