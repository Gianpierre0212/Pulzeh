use eframe::egui;
use std::time::Instant;
use sysinfo::System;

struct AplicacionPulzeh {
    sistema: System,

    // Guarda las últimas mediciones de CPU para calcular un promedio.
    historial_cpu: Vec<f32>,
    // Guarda la última medición instantánea de CPU.
    uso_cpu_actual: f32,

    // Guarda el momento de la última actualización del sistema.
    ultima_actualizacion: Instant,
}

impl AplicacionPulzeh {
    fn new() -> Self {
        let mut sistema = System::new_all();
        sistema.refresh_all();

        Self {
            sistema,
            historial_cpu: Vec::new(),
            uso_cpu_actual: 0.0,
            ultima_actualizacion: Instant::now(),
        }
    }
}

impl eframe::App for AplicacionPulzeh {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        // Solo actualizamos las mediciones una vez por segundo.
        if self.ultima_actualizacion.elapsed().as_secs() >= 1 {
            self.sistema.refresh_cpu_usage();
            self.sistema.refresh_memory();

            let uso_cpu = self.sistema.global_cpu_usage();
            self.uso_cpu_actual = uso_cpu;

            // Guardamos la medición actual en el historial.
            self.historial_cpu.push(uso_cpu);

            // Conservamos solamente las últimas 10 mediciones.
            if self.historial_cpu.len() > 10 {
                self.historial_cpu.remove(0);
            }

            // Reiniciamos el reloj para la siguiente actualización.
            self.ultima_actualizacion = Instant::now();
        }

        // Calculamos el promedio usando las mediciones guardadas.
        let suma_cpu: f32 = self.historial_cpu.iter().sum();

        let promedio_cpu = if self.historial_cpu.is_empty() {
            0.0
        } else {
            suma_cpu / self.historial_cpu.len() as f32
        };
        let memoria_total = self.sistema.total_memory();
        let memoria_usada = self.sistema.used_memory();
        // sysinfo entrega la memoria en bytes.
        // Convertimos los valores a gigabytes para mostrarlos correctamente.
        let memoria_total_gb = memoria_total as f32 / 1_073_741_824.0;
        let memoria_usada_gb = memoria_usada as f32 / 1_073_741_824.0;
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
                ui.label(format!("Uso actual: {:.1}%", self.uso_cpu_actual));
ui.label(format!("Promedio reciente: {promedio_cpu:.1}%"));

// La barra representa el uso instantáneo actual.
ui.add(
    egui::ProgressBar::new(self.uso_cpu_actual / 100.0)
        .show_percentage(),
);
            });

            ui.add_space(12.0);

            ui.group(|ui| {
                ui.heading("Memoria RAM");
                ui.label(format!(
                    "Uso actual: {porcentaje_memoria:.1}% ({memoria_usada_gb:.2} / {memoria_total_gb:.2} GB)"
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
