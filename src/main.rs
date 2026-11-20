mod aplicacion;
mod interfaz;
mod modelos;
mod sistema;

use aplicacion::AplicacionPulzeh;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 860.0])
            .with_min_inner_size([960.0, 680.0]),
        ..Default::default()
    };
    eframe::run_native(
        "PULZEH — Monitor de rendimiento del sistema",
        options,
        Box::new(|_contexto_creacion| Ok(Box::new(AplicacionPulzeh::nueva()))),
    )
}
