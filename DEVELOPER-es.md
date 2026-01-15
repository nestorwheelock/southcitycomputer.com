# Guía del Desarrollador

Documentación técnica para contribuir al proyecto del sitio web de South City Computer.

## Resumen de Arquitectura

```
┌─────────────────────────────────────────────────────────────┐
│                     Binario Rust Único                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │   Actix     │  │   Activos   │  │      Handlers       │  │
│  │   Web       │  │   Embebidos │  │        API          │  │
│  │   Server    │  │   (rust-    │  │                     │  │
│  │             │  │   embed)    │  │  /api/contact       │  │
│  │  Puerto 9000│  │             │  │  /view/contacts     │  │
│  │             │  │  HTML/CSS   │  │  /health            │  │
│  │             │  │  JS/Imágenes│  │                     │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     Sistema de Archivos                      │
│  ┌──────────────┐                                           │
│  │ contacts.csv │  ← Desacoplado del servidor web           │
│  │              │  ← Puede ser procesado por herramientas   │
│  └──────────────┘     externas                              │
└─────────────────────────────────────────────────────────────┘
```

### Principios de Diseño

1. **Despliegue de Binario Único**
   - Todos los activos compilados en el ejecutable
   - Sin dependencias de archivos externos en runtime
   - Simplifica el despliegue y reduce modos de fallo

2. **Activos Residentes en Memoria**
   - Archivos estáticos servidos desde RAM
   - Cero I/O de disco para contenido estático
   - Tiempos de respuesta sub-milisegundo consistentes

3. **Almacenamiento de Datos Desacoplado**
   - Formulario de contacto escribe a archivo CSV
   - Procesos externos manejan correo, base de datos, tickets
   - Reduce superficie de ataque del servidor web

4. **Paridad Desarrollo/Producción**
   - Misma API en servidores dev y prod
   - Servidor dev solo lee del disco en lugar de memoria

## Organización del Código

```
contact-handler/src/
├── main.rs           # Servidor de producción con activos embebidos
├── dev_server.rs     # Servidor de desarrollo (lectura de disco)
├── desktop.rs        # App de escritorio con servidor embebido
├── benchmark.rs      # Pruebas de rendimiento del lado del servidor
└── perf_client.rs    # Diagnósticos de red del lado del cliente
```

### main.rs - Servidor de Producción

Componentes clave:

```rust
// Embedding de activos
#[derive(RustEmbed)]
#[folder = "../"]
#[include = "*.html"]
#[include = "css/*.min.css"]
#[include = "js/*.min.js"]
#[include = "images/*.webp"]
struct Asset;

// Handler de archivos estáticos
async fn serve_static(path: web::Path<String>) -> impl Responder {
    // Servir desde activos embebidos con tipos MIME apropiados
}

// API de formulario de contacto
async fn submit_contact(form: web::Json<ContactForm>) -> impl Responder {
    // Validar, generar UUID, escribir a CSV
}
```

### dev_server.rs - Servidor de Desarrollo

Diferencias con producción:

- Lee archivos del disco en cada request
- Sin headers de caché (siempre fresco)
- Compilación más rápida (sin paso de embedding)
- Mismos endpoints API para consistencia

## Desarrollo Guiado por Pruebas

### Ciclo TDD

```
1. Escribir una prueba que falle
   └── cargo test -- --nocapture
       └── La prueba falla (esperado)

2. Escribir código mínimo para que pase
   └── cargo test
       └── La prueba pasa

3. Refactorizar
   └── cargo test
       └── Sigue pasando
```

### Escribir Pruebas

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_validation() {
        let form = ContactForm {
            name: "".to_string(),  // Inválido: vacío
            email: "test@ejemplo.com".to_string(),
            message: "Hola".to_string(),
        };
        assert!(validate_contact(&form).is_err());
    }

    #[test]
    fn test_valid_contact() {
        let form = ContactForm {
            name: "Juan".to_string(),
            email: "juan@ejemplo.com".to_string(),
            message: "Hola".to_string(),
        };
        assert!(validate_contact(&form).is_ok());
    }
}
```

### Categorías de Pruebas

| Tipo | Ubicación | Propósito |
|------|-----------|-----------|
| Unitaria | `src/*.rs` | Comportamiento a nivel de función |
| Integración | `tests/` | Comportamiento de endpoints API |
| Benchmark | `benches/` | Regresión de rendimiento |

### Ejecutar Pruebas

```bash
# Todas las pruebas
cargo test

# Prueba específica
cargo test test_contact_validation

# Con salida
cargo test -- --nocapture

# Solo pruebas de documentación
cargo test --doc
```

## Flujo de Trabajo de Desarrollo

### 1. Configuración

```bash
# Clonar repositorio
git clone https://github.com/southcitycomputer/southcitycomputer.com

# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Instalar Node (para minificación)
# Usa tu método preferido (nvm, apt, etc.)

# Instalar herramientas de minificación
npm install
```

### 2. Ciclo de Desarrollo

```bash
# Iniciar servidor dev (observa archivos)
cd contact-handler
cargo run --bin scc-dev

# En otra terminal, editar archivos
# Los cambios en HTML/CSS/JS aparecen al actualizar el navegador
```

### 3. Compilar Producción

```bash
# Minificar activos primero
npx cleancss -o css/style.min.css css/style.css
npx terser js/main.js -o js/main.min.js --compress --mangle

# Compilar binario de release
cd contact-handler
cargo build --release

# Binario en: target/release/scc-server
```

### 4. Pruebas

```bash
# Ejecutar todas las pruebas
cargo test

# Ejecutar con benchmarks
cargo test --release

# Verificar funcionalidad específica
cargo test health
cargo test contact
```

## Añadir Nuevas Funcionalidades

### Añadir un Nuevo Endpoint API

1. **Escribir pruebas primero**

```rust
#[cfg(test)]
mod tests {
    #[actix_web::test]
    async fn test_new_endpoint() {
        let app = test::init_service(
            App::new().route("/api/new", web::get().to(new_handler))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/new")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
```

2. **Ejecutar prueba (debería fallar)**

```bash
cargo test test_new_endpoint
# FALLÓ - el handler no existe
```

3. **Implementar handler**

```rust
async fn new_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}
```

4. **Ejecutar prueba (debería pasar)**

```bash
cargo test test_new_endpoint
# PASÓ
```

5. **Añadir al router**

```rust
.route("/api/new", web::get().to(new_handler))
```

### Añadir Nuevos Activos Estáticos

1. Colocar archivos en el directorio apropiado
2. Para producción, minificar si es necesario
3. Actualizar includes de `RustEmbed` si usas nuevo tipo de archivo
4. Recompilar binario de producción

```rust
#[derive(RustEmbed)]
#[folder = "../"]
#[include = "*.html"]
#[include = "css/*.min.css"]
#[include = "js/*.min.js"]
#[include = "images/*.webp"]
#[include = "fonts/*.woff2"]  // Nuevo tipo de archivo
struct Asset;
```

## Estilo de Código

### Rust

- Ejecutar `cargo fmt` antes de hacer commit
- Ejecutar `cargo clippy` para capturar problemas comunes
- Mantener funciones enfocadas y pequeñas
- Documentar APIs públicas

```rust
/// Valida un envío de formulario de contacto.
///
/// # Errores
/// Devuelve un error si el nombre está vacío o el email es inválido.
pub fn validate_contact(form: &ContactForm) -> Result<(), ValidationError> {
    // ...
}
```

### HTML/CSS/JS

- HTML5 semántico
- CSS responsive mobile-first
- JavaScript vanilla (sin frameworks)
- Mejora progresiva

## Depuración

### Logs del Servidor

```bash
# Desarrollo - logs a stdout
cargo run --bin scc-dev 2>&1 | tee server.log

# Producción - redirigir a archivo
./scc-server > server.log 2>&1
```

### Problemas Comunes

**¿Binario muy grande?**
- Verificar símbolos de depuración: `strip target/release/scc-server`
- Verificar que solo `.min.css`/`.min.js` están incluidos

**¿Activos no se actualizan?**
- Recompilar binario después de cambiar archivos embebidos
- Limpiar caché del navegador
- Usar servidor dev para iteración

**¿Formulario de contacto no funciona?**
- Verificar que `contacts.csv` es escribible
- Verificar headers CORS si es cross-origin
- Revisar consola del navegador por errores

## Pruebas de Rendimiento

### Validación Rápida

```bash
# Iniciar servidor
./target/release/scc-server &

# Verificación rápida
./target/release/scc-benchmark quick

# Suite completa
./target/release/scc-benchmark -n 100 -c 10
```

### Detección de Regresión

Antes de mergear cambios, comparar rendimiento:

```bash
# Base (main actual)
git checkout main
cargo build --release
./target/release/scc-benchmark > baseline.txt

# Tus cambios
git checkout feature-branch
cargo build --release
./target/release/scc-benchmark > feature.txt

# Comparar
diff baseline.txt feature.txt
```

## Dependencias

Dependencias principales (mantenidas al mínimo):

| Crate | Propósito |
|-------|-----------|
| actix-web | Framework de servidor HTTP |
| actix-cors | Middleware CORS |
| rust-embed | Embedding de activos |
| serde | Serialización |
| uuid | IDs únicos |
| chrono | Timestamps |

Opcionales:
| Crate | Propósito | Feature |
|-------|-----------|---------|
| wry | WebView | `desktop` |
| tao | Gestión de ventanas | `desktop` |

## Consideraciones de Seguridad

### Validación de Entrada

Siempre validar entrada del usuario:

```rust
fn validate_email(email: &str) -> bool {
    // Validación básica - considerar usar crate validator para producción
    email.contains('@') && email.contains('.')
}

fn sanitize_input(input: &str) -> String {
    // Remover caracteres potencialmente peligrosos
    input.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?@-_".contains(*c))
        .collect()
}
```

### Acceso al Sistema de Archivos

- Archivos CSV escritos con permisos restringidos
- Sin rutas de archivo controladas por usuario
- Validar operaciones de archivo

### Mejoras de Seguridad Futuras

Ver [ROADMAP.md](ROADMAP.md) para mejoras de seguridad planificadas incluyendo almacenamiento encriptado y hashing de contraseñas.

## Obtener Ayuda

- Abrir un issue en GitHub
- Revisar documentación existente
- Revisar archivos de prueba para ejemplos de uso
