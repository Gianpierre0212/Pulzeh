use eframe::egui;

/// Dibuja una barra de progreso con el valor y color recibidos.
pub fn barra_progreso(ui: &mut egui::Ui, valor: f32, color: egui::Color32) {
    ui.add(
        egui::ProgressBar::new(valor.clamp(0.0, 1.0))
            .fill(color)
            .show_percentage(),
    );
}
pub fn estado_recurso(porcentaje: f32) -> &'static str {
    if porcentaje < 60.0 {
        "Normal"
    } else if porcentaje < 85.0 {
        "Atención"
    } else {
        "Alto"
    }
}
pub fn color_estado(porcentaje: f32) -> egui::Color32 {
    if porcentaje < 60.0 {
        egui::Color32::from_rgb(66, 217, 208)
    } else if porcentaje < 85.0 {
        egui::Color32::from_rgb(235, 190, 70)
    } else {
        egui::Color32::from_rgb(220, 80, 80)
    }
}

/// Combina los estados de CPU y memoria en un estado general.
pub fn estado_general(estado_cpu: &str, estado_memoria: &str) -> &'static str {
    if estado_cpu == "Esperando" {
        "Esperando"
    } else if estado_cpu == "Alto" || estado_memoria == "Alto" {
        "Alto"
    } else if estado_cpu == "Atención" || estado_memoria == "Atención" {
        "Atención"
    } else {
        "Normal"
    }
}

/// Devuelve el color correspondiente a un estado general.
pub fn color_nombre_estado(estado: &str) -> egui::Color32 {
    match estado {
        "Alto" => egui::Color32::from_rgb(220, 80, 80),
        "Atención" => egui::Color32::from_rgb(235, 190, 70),
        "Normal" => egui::Color32::from_rgb(66, 217, 208),
        _ => egui::Color32::from_rgb(137, 149, 163),
    }
}
