use sysinfo::System;

/// Actualiza la información de memoria RAM.
pub fn actualizar(sistema: &mut System) {
    sistema.refresh_memory();
}

/// Devuelve la memoria usada, total y el porcentaje utilizado.
pub fn leer(sistema: &System) -> (u64, u64, f32) {
    let memoria_total = sistema.total_memory();
    let memoria_usada = sistema.used_memory();
    let porcentaje = calcular_porcentaje(memoria_usada, memoria_total);

    (memoria_usada, memoria_total, porcentaje)
}

/// Calcula el porcentaje de memoria utilizada de forma segura.
pub fn calcular_porcentaje(usada: u64, total: u64) -> f32 {
    if total > 0 {
        usada as f32 / total as f32 * 100.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod pruebas {
    use super::calcular_porcentaje;

    #[test]
    fn calcula_la_mitad_de_la_memoria() {
        assert_eq!(calcular_porcentaje(50, 100), 50.0);
    }

    #[test]
    fn devuelve_cero_si_no_hay_memoria_total() {
        assert_eq!(calcular_porcentaje(50, 0), 0.0);
    }
}
