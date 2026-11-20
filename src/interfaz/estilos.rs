use eframe::egui;

/// Aplica la paleta y los estados de interacción de la interfaz.
pub fn aplicar(contexto: &egui::Context) {
    let mut visuales = egui::Visuals::dark();
    visuales.panel_fill = color_fondo();
    visuales.window_fill = color_superficie();
    visuales.faint_bg_color = color_superficie_suave();
    visuales.extreme_bg_color = egui::Color32::from_rgb(10, 13, 17);
    visuales.code_bg_color = color_superficie_suave();
    visuales.override_text_color = Some(color_texto());
    visuales.hyperlink_color = color_acento();
    visuales.selection.bg_fill = color_seleccion();
    visuales.selection.stroke = egui::Stroke::new(1.0_f32, color_acento());
    visuales.window_stroke = egui::Stroke::new(1.0_f32, color_borde());
    visuales.widgets.noninteractive.bg_fill = color_superficie();
    visuales.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0_f32, color_texto());
    visuales.widgets.inactive.bg_fill = color_superficie_suave();
    visuales.widgets.inactive.fg_stroke = egui::Stroke::new(1.0_f32, color_texto());
    visuales.widgets.hovered.bg_fill = egui::Color32::from_rgb(47, 60, 69);
    visuales.widgets.hovered.fg_stroke = egui::Stroke::new(1.0_f32, color_texto());
    visuales.widgets.active.bg_fill = color_seleccion();
    visuales.widgets.active.fg_stroke = egui::Stroke::new(1.0_f32, color_acento());

    contexto.set_visuals(visuales);
    contexto.style_mut(|estilo| {
        estilo.spacing.item_spacing = egui::vec2(10.0, 8.0);
        estilo.spacing.button_padding = egui::vec2(10.0, 6.0);
        estilo.visuals.menu_corner_radius = egui::CornerRadius::same(10);
    });
}

pub fn color_fondo() -> egui::Color32 {
    egui::Color32::from_rgb(16, 19, 24)
}

pub fn color_barra_lateral() -> egui::Color32 {
    egui::Color32::from_rgb(19, 23, 29)
}

pub fn color_superficie() -> egui::Color32 {
    egui::Color32::from_rgb(25, 31, 39)
}

pub fn color_superficie_suave() -> egui::Color32 {
    egui::Color32::from_rgb(31, 39, 48)
}

pub fn color_borde() -> egui::Color32 {
    egui::Color32::from_rgb(48, 59, 69)
}

pub fn color_seleccion() -> egui::Color32 {
    egui::Color32::from_rgb(31, 59, 66)
}

pub fn color_texto() -> egui::Color32 {
    egui::Color32::from_rgb(231, 237, 242)
}

/// Color de acento principal para selección y actividad del procesador.
pub fn color_acento() -> egui::Color32 {
    egui::Color32::from_rgb(66, 217, 208)
}

/// Azul diferenciador para representar la memoria RAM.
pub fn color_memoria() -> egui::Color32 {
    egui::Color32::from_rgb(76, 145, 211)
}

/// Color secundario de etiquetas y texto de apoyo.
pub fn color_secundario() -> egui::Color32 {
    egui::Color32::from_rgb(151, 163, 176)
}
