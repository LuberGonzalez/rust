/// Función para generar un log con prefijo estandarizado
fn log_event(nombre: &str) {
    println!("[LOG] Acción realizada: {nombre}");
}

/// Función para simular exportación de datos
pub fn exportar_datos() {
    println!("Exportando datos al servidor remoto...");
    log_event("exportar");
}

/// Función para simular importación de datos
pub fn importar_datos() {
    println!("Importando datos desde archivo local...");
    log_event("importar");
}
