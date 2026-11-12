use eframe::egui;
use std::time::Instant;
use sysinfo::System;

use crate::{interfaz, modelos, sistema};

pub struct AplicacionPulzeh {
    sistema: System,
    // Indica la pantalla que está actualmente activa.
    vista_actual: modelos::Vista,

    // Guarda las últimas mediciones de CPU para calcular un promedio.
    historial_cpu: Vec<f32>,

    // Guarda los últimos porcentajes de memoria RAM.
    historial_memoria: Vec<f32>,

    // Guarda el último porcentaje de memoria calculado.
    porcentaje_memoria_actual: f32,

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
            // Indica la pantalla que está actualmente activa.
            vista_actual: modelos::Vista::Dashboard,
            historial_cpu: Vec::new(),
            historial_memoria: Vec::new(),
            porcentaje_memoria_actual: 0.0,
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

            let (_, _, porcentaje_memoria) = sistema::memoria::leer(&self.sistema);
            self.porcentaje_memoria_actual = porcentaje_memoria;

            // Guardamos el porcentaje de memoria en el historial.
            self.historial_memoria.push(porcentaje_memoria);

            // Conservamos las últimas 60 mediciones.
            if self.historial_memoria.len() > 60 {
                self.historial_memoria.remove(0);
            }

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

        let nombre_sistema = System::name().unwrap_or_else(|| "No disponible".to_string());
        let version_sistema = System::os_version().unwrap_or_else(|| "No disponible".to_string());

        let estado_cpu = if self.historial_cpu.is_empty() {
            "Esperando"
        } else {
            interfaz::componentes::estado_recurso(self.uso_cpu_actual)
        };
        let estado_memoria = interfaz::componentes::estado_recurso(porcentaje_memoria);
        let estado_general = interfaz::componentes::estado_general(estado_cpu, estado_memoria);

        if let Some(nueva_vista) = interfaz::panel_principal::mostrar(
            contexto,
            self.historial_cpu.is_empty(),
            self.uso_cpu_actual,
            promedio_cpu,
            porcentaje_memoria,
            memoria_usada_gb,
            memoria_total_gb,
            estado_cpu,
            estado_memoria,
            estado_general,
            &self.historial_cpu,
            &self.historial_memoria,
            &nombre_sistema,
            &version_sistema,
            self.vista_actual,
        ) {
            self.vista_actual = nueva_vista;
        }

        contexto.request_repaint_after(std::time::Duration::from_secs(1));
    }
}
