use sysinfo::System;

/// Actualiza la información del procesador y devuelve su uso global.
pub fn actualizar(sistema: &mut System) -> f32 {
    sistema.refresh_cpu_usage();
    sistema.global_cpu_usage()
}
