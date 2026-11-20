use crate::modelos::Vista;
use crate::sistema::red::EstadisticasRed;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

const ESPACIADO_SECCION: f32 = 15.0;

/// Dibuja la estructura de navegación y la pantalla activa de PULZEH.
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

    egui::SidePanel::left("navegacion_pulzeh")
        .resizable(false)
        .exact_width(224.0)
        .frame(
            egui::Frame::new()
                .fill(super::estilos::color_barra_lateral())
                .inner_margin(egui::Margin::same(18))
                .stroke(egui::Stroke::new(1.0_f32, super::estilos::color_borde())),
        )
        .show(contexto, |ui| {
            ui.vertical(|ui| {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    egui::Frame::new()
                        .fill(super::estilos::color_acento())
                        .corner_radius(egui::CornerRadius::same(9))
                        .inner_margin(egui::Margin::symmetric(10, 6))
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new("P")
                                    .size(20.0)
                                    .strong()
                                    .color(super::estilos::color_fondo()),
                            );
                        });
                    ui.label(
                        egui::RichText::new("PULZEH")
                            .size(19.0)
                            .strong()
                            .color(super::estilos::color_texto()),
                    );
                });
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("MONITOR DEL SISTEMA")
                        .small()
                        .color(super::estilos::color_secundario()),
                );

                ui.add_space(34.0);
                ui.label(
                    egui::RichText::new("ESPACIO DE TRABAJO")
                        .small()
                        .strong()
                        .color(super::estilos::color_secundario()),
                );
                ui.add_space(10.0);

                if boton_navegacion(ui, "◫", "Resumen", vista_actual == Vista::Dashboard) {
                    nueva_vista = Some(Vista::Dashboard);
                }
                ui.add_space(5.0);
                if boton_navegacion(ui, "▦", "Hardware", vista_actual == Vista::Hardware) {
                    nueva_vista = Some(Vista::Hardware);
                }
                ui.add_space(5.0);
                if boton_navegacion(ui, "≋", "Procesos", vista_actual == Vista::Procesos) {
                    nueva_vista = Some(Vista::Procesos);
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.label(
                        egui::RichText::new("Versión 0.2.0")
                            .small()
                            .color(super::estilos::color_secundario()),
                    );
                    ui.label(
                        egui::RichText::new("MEDICIÓN LOCAL · WINDOWS")
                            .small()
                            .color(super::estilos::color_secundario()),
                    );
                    ui.add_space(8.0);
                    ui.separator();
                });
            });
        });

    egui::CentralPanel::default()
        .frame(
            egui::Frame::new()
                .fill(super::estilos::color_fondo())
                .inner_margin(egui::Margin::same(24)),
        )
        .show(contexto, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("contenido_principal")
                .auto_shrink([false, false])
                .show(ui, |ui| match vista_actual {
                    Vista::Dashboard => {
                        mostrar_dashboard(
                            ui,
                            cpu_sin_datos,
                            uso_cpu_actual,
                            promedio_cpu,
                            porcentaje_memoria,
                            memoria_usada_gb,
                            memoria_total_gb,
                            estado_cpu,
                            estado_memoria,
                            estado_general,
                            historial_cpu,
                            historial_memoria,
                            nombre_sistema,
                            version_sistema,
                            estadisticas_red,
                        );
                    }
                    Vista::Hardware => mostrar_hardware(
                        ui,
                        sistema,
                        nombre_sistema,
                        version_sistema,
                        memoria_usada_gb,
                        memoria_total_gb,
                    ),
                    Vista::Procesos => mostrar_pantalla_procesos(ui, sistema, filtro_procesos),
                });
        });

    nueva_vista
}

fn boton_navegacion(ui: &mut egui::Ui, icono: &str, titulo: &str, seleccionado: bool) -> bool {
    let relleno = if seleccionado {
        super::estilos::color_seleccion()
    } else {
        egui::Color32::TRANSPARENT
    };
    let texto = if seleccionado {
        super::estilos::color_acento()
    } else {
        super::estilos::color_texto()
    };

    egui::Frame::new()
        .fill(relleno)
        .corner_radius(egui::CornerRadius::same(9))
        .inner_margin(egui::Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.add_sized(
                [ui.available_width(), 24.0],
                egui::Button::new(
                    egui::RichText::new(format!("{icono}    {titulo}"))
                        .color(texto)
                        .size(14.0),
                )
                .frame(false),
            )
            .clicked()
        })
        .inner
}

#[allow(clippy::too_many_arguments)]
fn mostrar_dashboard(
    ui: &mut egui::Ui,
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
) {
    mostrar_cabecera(
        ui,
        "Resumen del sistema",
        &format!("{nombre_sistema} {version_sistema}"),
        Some(estado_general),
        true,
    );
    ui.add_space(ESPACIADO_SECCION);

    ui.columns(2, |columnas| {
        tarjeta_recurso(
            &mut columnas[0],
            "CPU",
            uso_cpu_actual,
            &format!("Promedio (1 min): {promedio_cpu:.1}%"),
            estado_cpu,
            super::estilos::color_acento(),
            cpu_sin_datos,
        );
        tarjeta_recurso(
            &mut columnas[1],
            "Memoria RAM",
            porcentaje_memoria,
            &format!("Usada: {memoria_usada_gb:.2} GB / {memoria_total_gb:.2} GB"),
            estado_memoria,
            super::estilos::color_memoria(),
            false,
        );
    });

    ui.add_space(ESPACIADO_SECCION);
    mostrar_tarjeta_red(ui, estadisticas_red);

    ui.add_space(ESPACIADO_SECCION);
    ui.heading(egui::RichText::new("Tendencias").size(19.0).strong());
    ui.add_space(10.0);

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
        tarjeta_grafico(
            &mut columnas[0],
            "CPU",
            "historial_cpu",
            puntos_cpu,
            super::estilos::color_acento(),
            historial_cpu.last().copied().unwrap_or(uso_cpu_actual),
        );
        tarjeta_grafico(
            &mut columnas[1],
            "Memoria RAM",
            "historial_memoria",
            puntos_memoria,
            super::estilos::color_memoria(),
            historial_memoria
                .last()
                .copied()
                .unwrap_or(porcentaje_memoria),
        );
    });
}

fn mostrar_cabecera(
    ui: &mut egui::Ui,
    titulo: &str,
    subtitulo: &str,
    estado: Option<&str>,
    mostrar_actualizacion: bool,
) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.heading(egui::RichText::new(titulo).size(26.0).strong());
            ui.label(egui::RichText::new(subtitulo).color(super::estilos::color_secundario()));
        });

        if let Some(estado) = estado {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                insignia_estado(ui, estado);
                if mostrar_actualizacion {
                    ui.add_space(12.0);
                    ui.label(
                        egui::RichText::new("↻  Actualiza cada 1 s")
                            .color(super::estilos::color_secundario()),
                    );
                }
            });
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn tarjeta_recurso(
    ui: &mut egui::Ui,
    nombre: &str,
    porcentaje: f32,
    detalle: &str,
    estado: &str,
    color: egui::Color32,
    sin_datos: bool,
) {
    tarjeta(ui, |ui| {
        ui.horizontal(|ui| {
            let icono = if nombre == "CPU" { "▦" } else { "▤" };
            egui::Frame::new()
                .fill(color.gamma_multiply(0.16))
                .corner_radius(egui::CornerRadius::same(8))
                .inner_margin(egui::Margin::same(6))
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(icono).size(18.0).strong().color(color));
                });
            ui.label(egui::RichText::new(nombre).size(17.0).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let color_estado = if estado == "Normal" {
                    color
                } else {
                    super::componentes::color_nombre_estado(estado)
                };
                insignia_estado_con_color(ui, estado, color_estado);
            });
        });
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if sin_datos {
                ui.label(
                    egui::RichText::new("Midiendo…")
                        .size(34.0)
                        .strong()
                        .color(super::estilos::color_secundario()),
                );
            } else {
                ui.label(
                    egui::RichText::new(format!("{porcentaje:.1}%"))
                        .size(42.0)
                        .strong()
                        .color(color),
                );
            }
        });
        ui.add_space(4.0);
        super::componentes::barra_progreso(ui, porcentaje / 100.0, color);
        ui.add_space(7.0);
        ui.label(
            egui::RichText::new(detalle)
                .small()
                .color(super::estilos::color_secundario()),
        );
    });
}

fn tarjeta_grafico(
    ui: &mut egui::Ui,
    titulo: &str,
    id: &'static str,
    puntos: PlotPoints,
    color: egui::Color32,
    valor_actual: f32,
) {
    tarjeta(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("▦").strong().color(color));
            ui.label(egui::RichText::new(titulo).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::Frame::new()
                    .fill(color.gamma_multiply(0.13))
                    .corner_radius(egui::CornerRadius::same(8))
                    .inner_margin(egui::Margin::symmetric(8, 4))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(format!("{valor_actual:.1}%"))
                                .small()
                                .strong()
                                .color(color),
                        );
                    });
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Último minuto")
                        .small()
                        .color(super::estilos::color_secundario()),
                );
            });
        });
        ui.add_space(7.0);
        Plot::new(id)
            .height(205.0)
            .include_y(0.0)
            .include_y(100.0)
            .show_axes([false, true])
            .show_grid([true, true])
            .allow_zoom(false)
            .allow_drag(false)
            .show(ui, |plot| {
                plot.line(Line::new(titulo, puntos).color(color).width(2.2_f32));
            });
    });
}

fn mostrar_hardware(
    ui: &mut egui::Ui,
    sistema: &sysinfo::System,
    nombre_sistema: &str,
    version_sistema: &str,
    memoria_usada_gb: f32,
    memoria_total_gb: f32,
) {
    mostrar_cabecera(
        ui,
        "Hardware",
        "Información detectada del equipo",
        None,
        false,
    );
    ui.add_space(ESPACIADO_SECCION);

    let modelo_cpu = sistema
        .cpus()
        .first()
        .map(|cpu| cpu.brand().trim())
        .filter(|marca| !marca.is_empty())
        .unwrap_or("No disponible");
    let nucleos_logicos = sistema.cpus().len();
    let nucleos_fisicos = sysinfo::System::physical_core_count()
        .map(|cantidad| cantidad.to_string())
        .unwrap_or_else(|| "No disponible".to_string());

    ui.columns(2, |columnas| {
        tarjeta_detalle(
            &mut columnas[0],
            "PROCESADOR",
            modelo_cpu,
            &format!("{nucleos_fisicos} núcleos físicos · {nucleos_logicos} lógicos"),
            super::estilos::color_acento(),
        );
        tarjeta_detalle(
            &mut columnas[1],
            "MEMORIA RAM",
            &format!("{memoria_total_gb:.2} GB detectados"),
            &format!("En uso: {memoria_usada_gb:.2} GB"),
            super::estilos::color_memoria(),
        );
    });

    ui.add_space(ESPACIADO_SECCION);
    tarjeta(ui, |ui| {
        ui.label(
            egui::RichText::new("SISTEMA OPERATIVO")
                .small()
                .strong()
                .color(super::estilos::color_acento()),
        );
        ui.add_space(8.0);
        ui.label(egui::RichText::new(nombre_sistema).size(18.0).strong());
        ui.label(
            egui::RichText::new(format!("Versión {version_sistema}"))
                .color(super::estilos::color_secundario()),
        );
    });

    ui.add_space(ESPACIADO_SECCION);
    ui.label(egui::RichText::new("Más componentes").size(19.0).strong());
    ui.add_space(9.0);
    ui.columns(2, |columnas| {
        tarjeta_pendiente(
            &mut columnas[0],
            "Gráficos",
            "La detección de GPU se añadirá en una etapa posterior.",
        );
        tarjeta_pendiente(
            &mut columnas[1],
            "Almacenamiento",
            "La lectura de unidades y espacio libre está pendiente.",
        );
    });
}

fn tarjeta_detalle(
    ui: &mut egui::Ui,
    categoria: &str,
    valor: &str,
    detalle: &str,
    color: egui::Color32,
) {
    tarjeta(ui, |ui| {
        ui.label(egui::RichText::new(categoria).small().strong().color(color));
        ui.add_space(9.0);
        ui.label(egui::RichText::new(valor).size(18.0).strong());
        ui.add_space(3.0);
        ui.label(
            egui::RichText::new(detalle)
                .small()
                .color(super::estilos::color_secundario()),
        );
    });
}

fn tarjeta_pendiente(ui: &mut egui::Ui, titulo: &str, detalle: &str) {
    tarjeta(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(titulo).strong());
            ui.label(
                egui::RichText::new("EN PREPARACIÓN")
                    .small()
                    .color(super::estilos::color_secundario()),
            );
        });
        ui.add_space(6.0);
        ui.label(
            egui::RichText::new(detalle)
                .small()
                .color(super::estilos::color_secundario()),
        );
    });
}

fn mostrar_pantalla_procesos(ui: &mut egui::Ui, sistema: &sysinfo::System, filtro: &mut String) {
    mostrar_cabecera(
        ui,
        "Procesos activos",
        "Consumo medido por el sistema · actualización cada segundo",
        None,
        false,
    );
    ui.add_space(ESPACIADO_SECCION);

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
    let cantidad_procesos = procesos.len();

    tarjeta(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Buscar proceso").strong());
            ui.add_space(10.0);
            ui.add_sized(
                [ui.available_width().min(360.0), 32.0],
                egui::TextEdit::singleline(filtro).hint_text("Nombre del proceso..."),
            );
        });
    });
    ui.add_space(12.0);

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(format!("{cantidad_procesos} procesos encontrados")).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new("Ordenados por uso de CPU")
                    .small()
                    .color(super::estilos::color_secundario()),
            );
        });
    });
    ui.add_space(8.0);

    tarjeta(ui, |ui| {
        egui::Grid::new("tabla_procesos")
            .striped(true)
            .num_columns(4)
            .min_col_width(60.0)
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

/// Presenta el resumen de red y mantiene los detalles técnicos plegados.
fn mostrar_tarjeta_red(ui: &mut egui::Ui, estadisticas: &EstadisticasRed) {
    tarjeta(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("⌁")
                    .size(24.0)
                    .strong()
                    .color(super::estilos::color_acento()),
            );
            ui.label(egui::RichText::new("Conectividad").size(19.0).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "{} adaptadores detectados",
                        estadisticas.interfaces
                    ))
                    .small()
                    .color(super::estilos::color_secundario()),
                );
            });
        });

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            let ancho_total = ui.available_width();
            let ancho_detalles = (ancho_total * 0.27).clamp(194.0, 254.0);
            let ancho_metricas = (ancho_total - ancho_detalles - 14.0).max(420.0);

            ui.allocate_ui_with_layout(
                egui::vec2(ancho_metricas, 76.0),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.columns(4, |columnas| {
                        metrica_red(
                            &mut columnas[0],
                            "↓",
                            "Descarga",
                            &formatear_velocidad_red(estadisticas.descarga_bytes_segundo),
                            super::estilos::color_acento(),
                        );
                        metrica_red(
                            &mut columnas[1],
                            "↑",
                            "Subida",
                            &formatear_velocidad_red(estadisticas.subida_bytes_segundo),
                            super::estilos::color_memoria(),
                        );
                        metrica_red(
                            &mut columnas[2],
                            "↓",
                            "Datos recibidos",
                            &formatear_bytes(estadisticas.recibidos),
                            super::estilos::color_acento(),
                        );
                        metrica_red(
                            &mut columnas[3],
                            "↑",
                            "Datos enviados",
                            &formatear_bytes(estadisticas.enviados),
                            super::estilos::color_memoria(),
                        );
                    });
                },
            );
            ui.separator();
            ui.allocate_ui_with_layout(
                egui::vec2(ancho_detalles, 76.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.collapsing("Adaptadores de red", |ui| {
                        mostrar_detalle_adaptadores(ui, estadisticas);
                    });
                    ui.collapsing("Conexiones TCP por aplicación", |ui| {
                        mostrar_detalle_conexiones(ui, estadisticas);
                    });
                },
            );
        });
    });
}

fn metrica_red(ui: &mut egui::Ui, flecha: &str, titulo: &str, valor: &str, color: egui::Color32) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(flecha).size(17.0).strong().color(color));
            ui.label(
                egui::RichText::new(titulo)
                    .small()
                    .color(super::estilos::color_secundario()),
            );
        });
        ui.add_space(3.0);
        ui.label(egui::RichText::new(valor).size(18.0).strong());
    });
}

fn mostrar_detalle_adaptadores(ui: &mut egui::Ui, estadisticas: &EstadisticasRed) {
    if estadisticas.detalles_interfaces.is_empty() {
        ui.label("No se detectaron adaptadores de red.");
    }
    for interfaz in &estadisticas.detalles_interfaces {
        ui.push_id(&interfaz.nombre, |ui| {
            ui.collapsing(format!("{} · {}", interfaz.nombre, interfaz.tipo), |ui| {
                ui.label(format!("Estado: {}", interfaz.estado));
                ui.label(if interfaz.activa {
                    "Actividad reciente detectada"
                } else {
                    "Sin actividad reciente"
                });
                ui.label(format!(
                    "Velocidad de enlace: ↓ {} / ↑ {}",
                    formatear_bits_por_segundo(interfaz.velocidad_bajada_bps),
                    formatear_bits_por_segundo(interfaz.velocidad_subida_bps)
                ));
                ui.label(format!(
                    "Recibidos: {} · Enviados: {}",
                    formatear_bytes(interfaz.recibidos),
                    formatear_bytes(interfaz.enviados)
                ));
                ui.label(format!(
                    "Errores: ↓ {} · ↑ {}",
                    interfaz.errores_recibidos, interfaz.errores_enviados
                ));
            });
        });
    }
}

fn mostrar_detalle_conexiones(ui: &mut egui::Ui, estadisticas: &EstadisticasRed) {
    ui.label(
        egui::RichText::new(
            "Conexiones activas por proceso; no indica cuántos datos transfirió cada aplicación.",
        )
        .small()
        .color(super::estilos::color_secundario()),
    );
    if estadisticas.conexiones_tcp.is_empty() {
        ui.label("No se detectaron conexiones TCP activas.");
        return;
    }

    let mut conexiones_por_proceso = std::collections::BTreeMap::new();
    for conexion in &estadisticas.conexiones_tcp {
        conexiones_por_proceso
            .entry((conexion.pid, conexion.nombre_proceso.as_str()))
            .or_insert_with(Vec::new)
            .push(conexion);
    }

    egui::ScrollArea::vertical()
        .id_salt("lista_conexiones_tcp")
        .max_height(220.0)
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
                        },
                    );
                });
            }
        });
}

fn insignia_estado(ui: &mut egui::Ui, estado: &str) {
    insignia_estado_con_color(ui, estado, super::componentes::color_nombre_estado(estado));
}

fn insignia_estado_con_color(ui: &mut egui::Ui, estado: &str, color: egui::Color32) {
    egui::Frame::new()
        .fill(color.gamma_multiply(0.14))
        .corner_radius(egui::CornerRadius::same(20))
        .inner_margin(egui::Margin::symmetric(10, 5))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(format!("●  {estado}"))
                    .small()
                    .strong()
                    .color(color),
            );
        });
}

fn tarjeta(ui: &mut egui::Ui, contenido: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(super::estilos::color_superficie())
        .stroke(egui::Stroke::new(1.0_f32, super::estilos::color_borde()))
        .corner_radius(egui::CornerRadius::same(12))
        .inner_margin(egui::Margin::same(16))
        .show(ui, contenido);
}

fn formatear_bytes(bytes: u64) -> String {
    const BYTES_POR_MEGABYTE: f64 = 1_048_576.0;
    if bytes >= 1_073_741_824 {
        format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
    } else {
        format!("{:.2} MB", bytes as f64 / BYTES_POR_MEGABYTE)
    }
}

fn formatear_velocidad_red(bytes_por_segundo: f64) -> String {
    let bits_por_segundo = bytes_por_segundo * 8.0;
    if bits_por_segundo >= 1_000_000.0 {
        format!("{:.1} Mbps", bits_por_segundo / 1_000_000.0)
    } else if bits_por_segundo >= 1_000.0 {
        format!("{:.1} Kbps", bits_por_segundo / 1_000.0)
    } else {
        format!("{bits_por_segundo:.0} bps")
    }
}

fn formatear_bits_por_segundo(bits_por_segundo: u64) -> String {
    const BITS_POR_MEGABIT: f64 = 1_000_000.0;
    if bits_por_segundo >= 1_000_000 {
        format!("{:.1} Mbps", bits_por_segundo as f64 / BITS_POR_MEGABIT)
    } else if bits_por_segundo > 0 {
        format!("{:.0} Kbps", bits_por_segundo as f64 / 1_000.0)
    } else {
        "No disponible".to_string()
    }
}
