use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

/// Dibuja el panel principal de PULZEH.
#[allow(clippy::too_many_arguments)]
pub fn mostrar(
    contexto: &egui::Context,
    cpu_sin_datos: bool,
    uso_cpu_actual: f32,
    promedio_cpu: f32,
    porcentaje_memoria: f32,
    memoria_usada_gb: f32,
    memoria_total_gb: f32,
    estado_cpu: &str,
    estado_memoria: &str,
    estado_general: &str,
    historial_cpu: &[f32],
    historial_memoria: &[f32],
    nombre_sistema: &str,
    version_sistema: &str,
) {
    egui::SidePanel::left("barra_lateral")
        .resizable(false)
        .default_width(190.0)
        .show(contexto, |ui| {
            ui.add_space(12.0);
            ui.heading(egui::RichText::new("PULZEH").color(super::estilos::color_acento()));
            ui.label(
                egui::RichText::new("SYSTEM PERFORMANCE MONITOR")
                    .small()
                    .color(super::estilos::color_secundario()),
            );
            ui.add_space(24.0);

            ui.label(
                egui::RichText::new("NAVEGACIÓN")
                    .small()
                    .color(super::estilos::color_secundario()),
            );
            ui.add_space(6.0);
            ui.colored_label(super::estilos::color_acento(), "●  Dashboard");
            ui.add_space(8.0);
            ui.label("○  Hardware");
            ui.label(
                egui::RichText::new("   Próximamente")
                    .small()
                    .color(super::estilos::color_secundario()),
            );
            ui.add_space(8.0);
            ui.label("○  Procesos");
            ui.label(
                egui::RichText::new("   Próximamente")
                    .small()
                    .color(super::estilos::color_secundario()),
            );

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label(
                    egui::RichText::new("Versión 0.2.0")
                        .small()
                        .color(super::estilos::color_secundario()),
                );
            });
        });

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
        ui.label(
            egui::RichText::new(format!("Estado general: {estado_general}"))
                .color(super::componentes::color_nombre_estado(estado_general)),
        );
        ui.add_space(20.0);

        ui.group(|ui| {
            ui.heading("Información del equipo");
            ui.label(format!("Sistema operativo: {nombre_sistema}"));
            ui.label(format!("Versión: {version_sistema}"));
        });

        ui.add_space(16.0);

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

        ui.add_space(20.0);
        ui.heading("Historial de rendimiento");

        let puntos_cpu: PlotPoints = historial_cpu
            .iter()
            .enumerate()
            .map(|(indice, valor)| [indice as f64, *valor as f64])
            .collect();
        let puntos_memoria: PlotPoints = historial_memoria
            .iter()
            .enumerate()
            .map(|(indice, valor)| [indice as f64, *valor as f64])
            .collect();

        ui.columns(2, |columnas| {
            columnas[0].group(|ui| {
                ui.label("CPU");
                Plot::new("historial_cpu")
                    .height(180.0)
                    .include_y(0.0)
                    .include_y(100.0)
                    .show(ui, |plot_ui| {
                        plot_ui.line(
                            Line::new("CPU", puntos_cpu)
                                .color(super::estilos::color_acento()),
                        );
                    });
            });

            columnas[1].group(|ui| {
                ui.label("Memoria RAM");
                Plot::new("historial_memoria")
                    .height(180.0)
                    .include_y(0.0)
                    .include_y(100.0)
                    .show(ui, |plot_ui| {
                        plot_ui.line(
                            Line::new("RAM", puntos_memoria)
                                .color(super::estilos::color_memoria()),
                        );
                    });
            });
        });
    });
}
