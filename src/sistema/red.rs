use sysinfo::Networks;

/// Representa los datos básicos de la red.
pub struct EstadisticasRed {
    pub interfaces: usize,
    pub recibidos: u64,
    pub enviados: u64,
    pub descarga_bytes_segundo: f64,
    pub subida_bytes_segundo: f64,
    pub detalles_interfaces: Vec<InterfazRed>,
    pub conexiones_tcp: Vec<ConexionAplicacion>,
}

/// Conexión TCP vinculada por Windows a un proceso.
pub struct ConexionAplicacion {
    pub pid: u32,
    pub nombre_proceso: String,
    pub estado: &'static str,
    pub direccion_remota: String,
    pub puerto_remoto: u16,
}

/// Contadores acumulados de una interfaz detectada por el sistema.
pub struct InterfazRed {
    pub nombre: String,
    pub tipo: &'static str,
    pub activa: bool,
    pub estado: &'static str,
    pub velocidad_bajada_bps: u64,
    pub velocidad_subida_bps: u64,
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
            estado: "Estado físico no consultado",
            velocidad_bajada_bps: 0,
            velocidad_subida_bps: 0,
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
        conexiones_tcp: Vec::new(),
    }
}

/// Consulta conexiones TCP IPv4 e IPv6 y obtiene el PID propietario.
/// Esto cuenta conexiones; no mide bytes transferidos por aplicación.
#[cfg(windows)]
pub fn leer_conexiones_por_aplicacion(sistema: &sysinfo::System) -> Vec<ConexionAplicacion> {
    use std::collections::HashMap;

    let mut conexiones = Vec::new();
    leer_tabla_tcp_ipv4(&mut conexiones);
    leer_tabla_tcp_ipv6(&mut conexiones);

    let mut por_proceso: HashMap<(u32, String), Vec<ConexionAplicacion>> = HashMap::new();
    for mut conexion in conexiones {
        if let Some(proceso) = sistema.process(sysinfo::Pid::from_u32(conexion.pid)) {
            conexion.nombre_proceso = proceso.name().to_string_lossy().into_owned();
        }
        por_proceso
            .entry((conexion.pid, conexion.nombre_proceso.clone()))
            .or_default()
            .push(conexion);
    }

    let mut salida: Vec<_> = por_proceso.into_values().flatten().collect();
    salida.sort_by(|a, b| a.nombre_proceso.cmp(&b.nombre_proceso));
    salida
}

#[cfg(windows)]
fn leer_tabla_tcp_ipv6(salida: &mut Vec<ConexionAplicacion>) {
    use std::{ffi::c_void, mem::size_of, net::Ipv6Addr, slice};
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        GetExtendedTcpTable, MIB_TCP_STATE_ESTAB, MIB_TCP6ROW_OWNER_PID, MIB_TCP6TABLE_OWNER_PID,
        TCP_TABLE_OWNER_PID_CONNECTIONS,
    };

    let mut bytes_necesarios = 0u32;
    let familia = windows_sys::Win32::Networking::WinSock::AF_INET6 as u32;
    let resultado = unsafe {
        GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut bytes_necesarios,
            0,
            familia,
            TCP_TABLE_OWNER_PID_CONNECTIONS,
            0,
        )
    };
    if bytes_necesarios == 0 || resultado == 0 {
        return;
    }

    let mut buffer = vec![0u64; (bytes_necesarios as usize).div_ceil(size_of::<u64>())];
    let resultado = unsafe {
        GetExtendedTcpTable(
            buffer.as_mut_ptr().cast::<c_void>(),
            &mut bytes_necesarios,
            0,
            familia,
            TCP_TABLE_OWNER_PID_CONNECTIONS,
            0,
        )
    };
    if resultado != 0 {
        return;
    }

    let tabla = buffer.as_ptr().cast::<MIB_TCP6TABLE_OWNER_PID>();
    let filas = unsafe {
        let cantidad = (*tabla).dwNumEntries as usize;
        let inicio: *const MIB_TCP6ROW_OWNER_PID = std::ptr::addr_of!((*tabla).table).cast();
        slice::from_raw_parts(inicio, cantidad)
    };
    for fila in filas {
        salida.push(ConexionAplicacion {
            pid: fila.dwOwningPid,
            nombre_proceso: "Proceso no disponible".to_string(),
            estado: if fila.dwState as i32 == MIB_TCP_STATE_ESTAB {
                "Establecida"
            } else {
                "Conexión TCP"
            },
            direccion_remota: Ipv6Addr::from(fila.ucRemoteAddr).to_string(),
            puerto_remoto: u16::from_be(fila.dwRemotePort as u16),
        });
    }
}

#[cfg(windows)]
fn leer_tabla_tcp_ipv4(salida: &mut Vec<ConexionAplicacion>) {
    use std::{ffi::c_void, mem::size_of, slice};
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        GetExtendedTcpTable, MIB_TCP_STATE_ESTAB, MIB_TCPROW_OWNER_PID, MIB_TCPTABLE_OWNER_PID,
        TCP_TABLE_OWNER_PID_CONNECTIONS,
    };

    let mut bytes_necesarios = 0u32;
    let resultado = unsafe {
        GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut bytes_necesarios,
            0,
            windows_sys::Win32::Networking::WinSock::AF_INET as u32,
            TCP_TABLE_OWNER_PID_CONNECTIONS,
            0,
        )
    };
    if bytes_necesarios == 0 || resultado == 0 {
        return;
    }

    let mut buffer = vec![0u64; (bytes_necesarios as usize).div_ceil(size_of::<u64>())];
    let resultado = unsafe {
        GetExtendedTcpTable(
            buffer.as_mut_ptr().cast::<c_void>(),
            &mut bytes_necesarios,
            0,
            windows_sys::Win32::Networking::WinSock::AF_INET as u32,
            TCP_TABLE_OWNER_PID_CONNECTIONS,
            0,
        )
    };
    if resultado != 0 {
        return;
    }

    let tabla = buffer.as_ptr().cast::<MIB_TCPTABLE_OWNER_PID>();
    let filas = unsafe {
        let cantidad = (*tabla).dwNumEntries as usize;
        let inicio: *const MIB_TCPROW_OWNER_PID = std::ptr::addr_of!((*tabla).table).cast();
        slice::from_raw_parts(inicio, cantidad)
    };
    for fila in filas {
        salida.push(ConexionAplicacion {
            pid: fila.dwOwningPid,
            nombre_proceso: "Proceso no disponible".to_string(),
            estado: if fila.dwState as i32 == MIB_TCP_STATE_ESTAB {
                "Establecida"
            } else {
                "Conexión TCP"
            },
            direccion_remota: formatear_ipv4(fila.dwRemoteAddr),
            puerto_remoto: u16::from_be(fila.dwRemotePort as u16),
        });
    }
}

#[cfg(windows)]
fn formatear_ipv4(direccion: u32) -> String {
    let octetos = direccion.to_ne_bytes();
    format!(
        "{}.{}.{}.{}",
        octetos[0], octetos[1], octetos[2], octetos[3]
    )
}

#[cfg(not(windows))]
pub fn leer_conexiones_por_aplicacion(_sistema: &sysinfo::System) -> Vec<ConexionAplicacion> {
    Vec::new()
}

/// En Windows, reemplaza la clasificación aproximada por datos de la tabla de interfaces.
#[cfg(windows)]
pub fn enriquecer_desde_windows(estadisticas: &mut EstadisticasRed) {
    use std::slice;
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        FreeMibTable, GetIfTable2, MIB_IF_ROW2, MIB_IF_TABLE2,
    };
    use windows_sys::Win32::NetworkManagement::Ndis::{IfOperStatusUp, MediaConnectStateConnected};

    let mut tabla: *mut MIB_IF_TABLE2 = std::ptr::null_mut();
    // La API asigna la tabla; se libera abajo con FreeMibTable.
    if unsafe { GetIfTable2(&mut tabla) } != 0 || tabla.is_null() {
        return;
    }

    let filas = unsafe {
        let cantidad = (*tabla).NumEntries as usize;
        let inicio: *const MIB_IF_ROW2 = std::ptr::addr_of!((*tabla).Table).cast();
        slice::from_raw_parts(inicio, cantidad)
    };

    for fila in filas {
        let alias = convertir_utf16(&fila.Alias);
        let descripcion = convertir_utf16(&fila.Description);
        let entrada = estadisticas
            .detalles_interfaces
            .iter_mut()
            .find(|interfaz| {
                interfaz.nombre.eq_ignore_ascii_case(&alias)
                    || interfaz.nombre.eq_ignore_ascii_case(&descripcion)
                    || alias.eq_ignore_ascii_case(&interfaz.nombre)
            });

        if let Some(interfaz) = entrada {
            interfaz.nombre = if alias.is_empty() { descripcion } else { alias };
            interfaz.tipo = match fila.Type {
                6 => "Ethernet",
                71 => "Wi-Fi",
                24 => "Loopback",
                131 => "Túnel",
                _ => "Otro adaptador",
            };
            interfaz.estado = if fila.OperStatus == IfOperStatusUp
                && fila.MediaConnectState == MediaConnectStateConnected
            {
                "Conectado"
            } else if fila.OperStatus == IfOperStatusUp {
                "Activo; conexión física no confirmada"
            } else {
                "Desconectado o no disponible"
            };
            interfaz.velocidad_bajada_bps = fila.ReceiveLinkSpeed;
            interfaz.velocidad_subida_bps = fila.TransmitLinkSpeed;
        }
    }

    unsafe { FreeMibTable(tabla.cast()) };
}

#[cfg(not(windows))]
pub fn enriquecer_desde_windows(_estadisticas: &mut EstadisticasRed) {}

#[cfg(windows)]
fn convertir_utf16(texto: &[u16]) -> String {
    let fin = texto
        .iter()
        .position(|caracter| *caracter == 0)
        .unwrap_or(texto.len());
    String::from_utf16_lossy(&texto[..fin])
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
