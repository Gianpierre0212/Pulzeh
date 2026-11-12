mod aplicacion;
mod interfaz;
mod modelos;
mod sistema;

use aplicacion::AplicacionPulzeh;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "PULZEH — System Performance Monitor",
        options,
        Box::new(|_contexto_creacion| Ok(Box::new(AplicacionPulzeh::nueva()))),
    )
}
