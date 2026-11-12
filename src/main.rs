use eframe::egui;
use sysinfo::System;

struct AplicacionPulzeh {
    sistema: System,
}

impl AplicacionPulzeh {
    fn new() -> Self {
        let mut sistema = System::new_all();
        sistema.refresh_all();
        Self { sistema }
    }
}

impl eframe::App for AplicacionPulzeh {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        self.sistema.refresh_cpu_usage();
        self.sistema.refresh_memory();

        let uso_cpu = self.sistema.global_cpu_usage();
        let memoria_total = self.sistema.total_memory();
        let memoria_usada = self.sistema.used_memory();
        let porcentaje_memoria = if memoria_total > 0 {
            memoria_usada as f32 / memoria_total as f32 * 100.0
        } else {
            0.0
        };

        egui::CentralPanel::default().show(context, |ui| {
            ui.heading("PULZEH");
            ui.label("MONITOR DE RENDIMIENTO DEL SISTEMA");
            ui.add_space(20.0);

            ui.group(|ui| {
                ui.heading("CPU");
                ui.label(format!("Uso actual: {uso_cpu:.1}%"));
                ui.add(egui::ProgressBar::new(uso_cpu / 100.0).show_percentage());
            });

            ui.add_space(12.0);

            ui.group(|ui| {
                ui.heading("Memoria RAM");
                ui.label(format!(
                    "Uso actual: {porcentaje_memoria:.1}% ({memoria_usada} / {memoria_total} KB)"
                ));
                ui.add(egui::ProgressBar::new(porcentaje_memoria / 100.0).show_percentage());
            });
        });

        context.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "PULZEH — System Performance Monitor",
        options,
        Box::new(|_creation_context| Ok(Box::new(AplicacionPulzeh::new()))),
    )
}
