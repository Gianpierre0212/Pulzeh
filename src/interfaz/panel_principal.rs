use crate::modelos::Vista;
use crate::sistema::red::EstadisticasRed;
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
    estadisticas_red: &EstadisticasRed,
    vista_actual: Vista,
    sistema: &sysinfo::System,
    filtro_procesos: &mut String,
) -> Option<Vista> {
    let mut nueva_vista = None;

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
            if ui
                .selectable_label(vista_actual == Vista::Dashboard, "●  Dashboard")
                .clicked()
            {
                nueva_vista = Some(Vista::Dashboard);
            }
            ui.add_space(8.0);
            if ui
                .selectable_label(vista_actual == Vista::Hardware, "○  Hardware")
                .clicked()
            {
                nueva_vista = Some(Vista::Hardware);
            }
            ui.label(
                egui::RichText::new("   Sección activa")
                    .small()
                    .color(super::estilos::color_secundario()),
            );
            ui.add_space(8.0);
            if ui
                .selectable_label(vista_actual == Vista::Procesos, "○  Procesos")
                .clicked()
            {
                nueva_vista = Some(Vista::Procesos);
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label(
                    egui::RichText::new("Versión 0.2.0")
                        .small()
                        .color(super::estilos::color_secundario()),
                );
            });
        });

    if vista_actual == Vista::Hardware {
        egui::CentralPanel::default().show(contexto, |ui| {
            ui.heading("Hardware");
            ui.label("Información detallada del equipo");
            ui.add_space(20.0);
            ui.group(|ui| {
                ui.label("La sección de hardware está preparada.");
                ui.label("Añadiremos aquí CPU, GPU, almacenamiento y red.");
            });
            ui.add_space(12.0);
            mostrar_tarjeta_red(ui, estadisticas_red);
        });
        return nueva_vista;
    }

    if vista_actual == Vista::Procesos {
        egui::CentralPanel::default().show(contexto, |ui| {
            mostrar_pantalla_procesos(ui, sistema, filtro_procesos);
        });
        return nueva_vista;
    }

    egui::CentralPanel::default().show(contexto, |ui| {
        ui.heading("PULZEH");
        ui.label(egui::RichText::new("MONITOR DE RENDIMIENTO DEL SISTEMA")
            .color(super::estilos::color_secundario()));
        ui.label(egui::RichText::new("Actualización automática cada segundo").small()
            .color(super::estilos::color_secundario()));
        ui.separator();
        ui.label(egui::RichText::new(format!("Estado general: {estado_general}"))
            .color(super::componentes::color_nombre_estado(estado_general)));
        ui.add_space(20.0);
        ui.group(|ui| {
            ui.heading("Información del equipo");
            ui.label(format!("Sistema operativo: {nombre_sistema}"));
            ui.label(format!("Versión: {version_sistema}"));
        });
        ui.add_space(16.0);
        mostrar_tarjeta_red(ui, estadisticas_red);
        ui.add_space(16.0);
        ui.columns(2, |columnas| {
            columnas[0].group(|ui| {
                ui.heading(egui::RichText::new("CPU").color(super::estilos::color_acento()));
                if cpu_sin_datos { ui.label("Esperando primera medición..."); } else {
                    ui.label(format!("Uso actual: {uso_cpu_actual:.1}%"));
                    ui.label(format!("Promedio reciente: {promedio_cpu:.1}%"));
                    let color = super::componentes::color_estado(uso_cpu_actual);
                    ui.label(egui::RichText::new(format!("Estado: {estado_cpu}")).color(color));
                    super::componentes::barra_progreso(ui, uso_cpu_actual / 100.0, color);
                }
            });
            columnas[1].group(|ui| {
                ui.heading(egui::RichText::new("Memoria RAM").color(super::estilos::color_memoria()));
                ui.label(format!("Uso actual: {porcentaje_memoria:.1}% ({memoria_usada_gb:.2} / {memoria_total_gb:.2} GB)"));
                let color = super::componentes::color_estado(porcentaje_memoria);
                ui.label(egui::RichText::new(format!("Estado: {estado_memoria}")).color(color));
                super::componentes::barra_progreso(ui, porcentaje_memoria / 100.0, color);
            });
        });
        ui.add_space(20.0);
        ui.heading("Historial de rendimiento");
        let puntos_cpu: PlotPoints = historial_cpu.iter().enumerate()
            .map(|(i, v)| [i as f64, *v as f64]).collect();
        let puntos_memoria: PlotPoints = historial_memoria.iter().enumerate()
            .map(|(i, v)| [i as f64, *v as f64]).collect();
        ui.columns(2, |columnas| {
            columnas[0].group(|ui| {
                ui.label("CPU");
                Plot::new("historial_cpu").height(180.0).include_y(0.0).include_y(100.0)
                    .show(ui, |plot| plot.line(Line::new("CPU", puntos_cpu).color(super::estilos::color_acento())));
            });
            columnas[1].group(|ui| {
                ui.label("Memoria RAM");
                Plot::new("historial_memoria").height(180.0).include_y(0.0).include_y(100.0)
                    .show(ui, |plot| plot.line(Line::new("RAM", puntos_memoria).color(super::estilos::color_memoria())));
            });
        });
    });
    nueva_vista
}

fn mostrar_pantalla_procesos(ui: &mut egui::Ui, sistema: &sysinfo::System, filtro: &mut String) {
    ui.heading("Procesos activos");
    ui.label("Uso de CPU y memoria por proceso, actualizado cada segundo.");
    ui.add_space(10.0);
    ui.label("Buscar proceso:");
    ui.text_edit_singleline(filtro);

    let filtro_normalizado = filtro.to_lowercase();
    let mut procesos: Vec<_> = sistema
        .processes()
        .iter()
        .filter(|(_, proceso)| {
            proceso
                .name()
                .to_string_lossy()
                .to_lowercase()
                .contains(&filtro_normalizado)
        })
        .collect();
    procesos.sort_by(|(_, a), (_, b)| {
        b.cpu_usage()
            .total_cmp(&a.cpu_usage())
            .then_with(|| a.name().cmp(b.name()))
    });

    ui.label(format!("{} procesos encontrados", procesos.len()));
    ui.separator();
    egui::ScrollArea::vertical()
        .id_salt("lista_procesos")
        .show(ui, |ui| {
            egui::Grid::new("tabla_procesos")
                .striped(true)
                .num_columns(4)
                .show(ui, |ui| {
                    ui.strong("Proceso");
                    ui.strong("PID");
                    ui.strong("CPU");
                    ui.strong("Memoria");
                    ui.end_row();

                    for (pid, proceso) in procesos {
                        ui.label(proceso.name().to_string_lossy());
                        ui.label(pid.to_string());
                        ui.label(format!("{:.1}%", proceso.cpu_usage()));
                        ui.label(formatear_bytes(proceso.memory()));
                        ui.end_row();
                    }
                });
        });
}

fn mostrar_tarjeta_red(ui: &mut egui::Ui, estadisticas: &EstadisticasRed) {
    ui.group(|ui| {
        ui.heading("Actividad de red");
        ui.label(format!(
            "Interfaces detectadas: {}",
            estadisticas.interfaces
        ));
        ui.label(format!(
            "Recibidos acumulados: {}",
            formatear_bytes(estadisticas.recibidos)
        ));
        ui.label(format!(
            "Enviados acumulados: {}",
            formatear_bytes(estadisticas.enviados)
        ));
        ui.label(format!(
            "Descarga: {}/s",
            formatear_bytes_por_segundo(estadisticas.descarga_bytes_segundo)
        ));
        ui.label(format!(
            "Subida: {}/s",
            formatear_bytes_por_segundo(estadisticas.subida_bytes_segundo)
        ));
        ui.separator();
        ui.label("Detalle por adaptador");

        if estadisticas.detalles_interfaces.is_empty() {
            ui.label("No se detectaron interfaces de red.");
        } else {
            for interfaz in &estadisticas.detalles_interfaces {
                ui.collapsing(&interfaz.nombre, |ui| {
                    ui.label(format!("Tipo: {}", interfaz.tipo));
                    ui.label(format!("Estado: {}", interfaz.estado));
                    ui.label(if interfaz.activa {
                        "Actividad reciente: detectada"
                    } else {
                        "Actividad reciente: no detectada"
                    });
                    if interfaz.velocidad_bajada_bps > 0 || interfaz.velocidad_subida_bps > 0 {
                        ui.label(format!(
                            "Velocidad de enlace: ↓ {} / ↑ {}",
                            formatear_bits_por_segundo(interfaz.velocidad_bajada_bps),
                            formatear_bits_por_segundo(interfaz.velocidad_subida_bps),
                        ));
                    } else {
                        ui.label("Velocidad de enlace: no disponible");
                    }
                    ui.label(format!(
                        "Recibidos: {}",
                        formatear_bytes(interfaz.recibidos)
                    ));
                    ui.label(format!("Enviados: {}", formatear_bytes(interfaz.enviados)));
                    ui.label(format!(
                        "Errores al recibir: {}",
                        interfaz.errores_recibidos
                    ));
                    ui.label(format!("Errores al enviar: {}", interfaz.errores_enviados));
                });
            }
        }

        ui.separator();
        ui.label("Conexiones TCP IPv4/IPv6 por aplicación (no representa bytes transferidos):");
        if estadisticas.conexiones_tcp.is_empty() {
            ui.label("No se detectaron conexiones TCP activas.");
        } else {
            let mut conexiones_por_proceso = std::collections::BTreeMap::new();
            for conexion in &estadisticas.conexiones_tcp {
                conexiones_por_proceso
                    .entry((conexion.pid, conexion.nombre_proceso.as_str()))
                    .or_insert_with(Vec::new)
                    .push(conexion);
            }

            egui::ScrollArea::vertical()
                .id_salt("lista_conexiones_tcp")
                .max_height(240.0)
                .show(ui, |ui| {
                    for ((pid, nombre), conexiones) in conexiones_por_proceso {
                        ui.push_id(pid, |ui| {
                            ui.collapsing(
                                format!("{nombre} · PID {pid} · {} conexiones", conexiones.len()),
                                |ui| {
                                    for (indice, conexion) in conexiones.iter().enumerate() {
                                        ui.push_id(indice, |ui| {
                                            ui.label(format!(
                                                "{} · {}:{}",
                                                conexion.estado,
                                                conexion.direccion_remota,
                                                conexion.puerto_remoto
                                            ));
                                        });
                                    }
                                    ui.label("La tabla TCP no informa los bytes transferidos por aplicación.");
                                },
                            );
                        });
                    }
                });
        }
    });
}

fn formatear_bytes(bytes: u64) -> String {
    const BYTES_POR_MEGABYTE: f64 = 1_048_576.0;
    format!("{:.2} MB", bytes as f64 / BYTES_POR_MEGABYTE)
}

fn formatear_bytes_por_segundo(bytes_por_segundo: f64) -> String {
    const BYTES_POR_MEGABYTE: f64 = 1_048_576.0;
    format!("{:.2} MB", bytes_por_segundo / BYTES_POR_MEGABYTE)
}

fn formatear_bits_por_segundo(bits_por_segundo: u64) -> String {
    const BITS_POR_MEGABIT: f64 = 1_000_000.0;
    format!("{:.1} Mbps", bits_por_segundo as f64 / BITS_POR_MEGABIT)
}
