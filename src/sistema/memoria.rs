use sysinfo::System;

/// Actualiza la información de memoria RAM.
pub fn actualizar(sistema: &mut System) {
    sistema.refresh_memory();
}

/// Devuelve la memoria usada, total y el porcentaje utilizado.
pub fn leer(sistema: &System) -> (u64, u64, f32) {
    let memoria_total = sistema.total_memory();
    let memoria_usada = sistema.used_memory();
    let porcentaje = if memoria_total > 0 {
        memoria_usada as f32 / memoria_total as f32 * 100.0
    } else {
        0.0
    };

    (memoria_usada, memoria_total, porcentaje)
}
