use eframe::egui;
use std::time::Instant;
use sysinfo::System;

use crate::{interfaz, sistema};

pub struct AplicacionPulzeh {
    sistema: System,

    // Guarda las últimas mediciones de CPU para calcular un promedio.
    historial_cpu: Vec<f32>,

    // Guarda la última medición instantánea de CPU.
    uso_cpu_actual: f32,

    // Guarda el momento de la última actualización del sistema.
    ultima_actualizacion: Instant,
}

impl AplicacionPulzeh {
    pub fn nueva() -> Self {
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
    fn update(&mut self, contexto: &egui::Context, _marco: &mut eframe::Frame) {
        // Aplicamos el estilo visual definido en el módulo de interfaz.
        interfaz::estilos::aplicar(contexto);

        // Solo actualizamos las mediciones una vez por segundo.
        if self.ultima_actualizacion.elapsed().as_secs() >= 1 {
            let uso_cpu = sistema::cpu::actualizar(&mut self.sistema);
            sistema::memoria::actualizar(&mut self.sistema);
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

        let (memoria_usada, memoria_total, porcentaje_memoria) =
            sistema::memoria::leer(&self.sistema);

        // sysinfo entrega la memoria en bytes.
        // Convertimos los valores a gigabytes para mostrarlos correctamente.
        let memoria_total_gb = memoria_total as f32 / 1_073_741_824.0;
        let memoria_usada_gb = memoria_usada as f32 / 1_073_741_824.0;

        interfaz::panel_principal::mostrar(
            contexto,
            self.historial_cpu.is_empty(),
            self.uso_cpu_actual,
            promedio_cpu,
            porcentaje_memoria,
            memoria_usada_gb,
            memoria_total_gb,
        );

        contexto.request_repaint_after(std::time::Duration::from_secs(1));
    }
}
