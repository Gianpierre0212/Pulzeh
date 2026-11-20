use sysinfo::Networks;

/// Representa los datos básicos de la red.
pub struct EstadisticasRed {
    pub interfaces: usize,
    pub recibidos: u64,
    pub enviados: u64,
    pub descarga_bytes_segundo: f64,
    pub subida_bytes_segundo: f64,
    pub detalles_interfaces: Vec<InterfazRed>,
}

/// Contadores acumulados de una interfaz detectada por el sistema.
pub struct InterfazRed {
    pub nombre: String,
    pub tipo: &'static str,
    pub activa: bool,
    pub recibidos: u64,
    pub enviados: u64,
    pub errores_recibidos: u64,
    pub errores_enviados: u64,
}

/// Actualiza la información de las interfaces de red.
pub fn actualizar(redes: &mut Networks) {
    redes.refresh(true);
}

/// Lee los datos acumulados de todas las interfaces.
pub fn leer(redes: &Networks) -> EstadisticasRed {
    let mut recibidos = 0;
    let mut enviados = 0;
    let mut detalles_interfaces = Vec::with_capacity(redes.len());

    for (nombre, red) in redes {
        recibidos += red.received();
        enviados += red.transmitted();
        detalles_interfaces.push(InterfazRed {
            nombre: nombre.clone(),
            tipo: clasificar_interfaz(nombre),
            activa: red.received() > 0 || red.transmitted() > 0,
            recibidos: red.total_received(),
            enviados: red.total_transmitted(),
            errores_recibidos: red.total_errors_on_received(),
            errores_enviados: red.total_errors_on_transmitted(),
        });
    }

    detalles_interfaces.sort_by(|a, b| a.nombre.cmp(&b.nombre));

    EstadisticasRed {
        interfaces: redes.len(),
        recibidos,
        enviados,
        descarga_bytes_segundo: 0.0,
        subida_bytes_segundo: 0.0,
        detalles_interfaces,
    }
}

/// Clasifica por nombre cuando el controlador expone una etiqueta conocida.
/// Es una pista orientativa; Windows puede usar nombres de fabricante distintos.
fn clasificar_interfaz(nombre: &str) -> &'static str {
    let nombre = nombre.to_ascii_lowercase();
    if nombre.contains("wi-fi") || nombre.contains("wifi") || nombre.contains("wireless") {
        "Posible Wi-Fi (según nombre)"
    } else if nombre.contains("ethernet") || nombre.contains("gigabit") {
        "Posible Ethernet (según nombre)"
    } else if nombre.contains("loopback") || nombre.contains("virtual") || nombre.contains("vpn") {
        "Virtual o VPN (según nombre)"
    } else {
        "Tipo no identificado"
    }
}

/// Calcula tasas de transferencia a partir de dos lecturas y el tiempo transcurrido.
pub fn calcular_velocidades(
    lectura_anterior: &EstadisticasRed,
    lectura_actual: &EstadisticasRed,
    segundos: f64,
) -> (f64, f64) {
    if segundos <= 0.0 {
        return (0.0, 0.0);
    }

    let recibidos = lectura_actual
        .recibidos
        .saturating_sub(lectura_anterior.recibidos);
    let enviados = lectura_actual
        .enviados
        .saturating_sub(lectura_anterior.enviados);

    (recibidos as f64 / segundos, enviados as f64 / segundos)
}
