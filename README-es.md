# Sitio Web de South City Computer

Un servidor web de alto rendimiento, de un solo binario, construido con Rust. Logra cargas de página 46 veces más rápidas que las configuraciones tradicionales de WordPress a través de activos embebidos y arquitectura residente en memoria.

## Filosofía

Este proyecto demuestra que existen mejores alternativas a WordPress para muchos sitios web. Nuestro enfoque:

- **Binario monolítico**: Todo HTML, CSS, JS e imágenes embebidos en un solo ejecutable
- **Almacenamiento de datos desacoplado**: Los envíos del formulario de contacto se guardan en archivos CSV que pueden ser procesados por procesos externos (importación a base de datos, notificación por correo, creación de tickets) - manteniendo la lógica de negocio separada de la superficie de ataque web
- **Rendimiento primero**: Tiempos de respuesta sub-milisegundo, 26,000-58,000 req/s de throughput
- **Despliegue simple**: Copia un archivo, ejecútalo

## Inicio Rápido

```bash
# Compilar el servidor de producción
cd contact-handler
cargo build --release

# Ejecutarlo
./target/release/scc-server
# Servidor ejecutándose en http://127.0.0.1:9000
```

## Binarios

| Binario | Propósito | Uso |
|---------|-----------|-----|
| `scc-server` | Servidor web de producción | Activos embebidos, optimizado para velocidad |
| `scc-dev` | Servidor de desarrollo | Lee archivos del disco, actualización instantánea |
| `scc-desktop` | Wrapper de app de escritorio | Ventana nativa con servidor embebido |
| `scc-benchmark` | Benchmarks del servidor | Medir throughput y latencia |
| `scc-perf-client` | Diagnósticos del cliente | Timing de red, visualización de traceroute |

### Servidor de Producción (scc-server)

El binario principal de producción con todos los activos embebidos en memoria.

```bash
cargo build --release
./target/release/scc-server

# Opciones:
# PORT=8080 ./target/release/scc-server  # Puerto personalizado
```

Características:
- Todos los activos estáticos servidos desde memoria (cero I/O de disco)
- Compresión Gzip/Brotli para activos de texto
- API de formulario de contacto con almacenamiento CSV
- Endpoint de health check en `/health`

### Servidor de Desarrollo (scc-dev)

Lee archivos del disco para iteración rápida. No requiere recompilación al cambiar HTML/CSS/JS.

```bash
cargo build
./target/debug/scc-dev

# Opciones:
# STATIC_DIR=/ruta/a/activos ./target/debug/scc-dev
# PORT=3000 ./target/debug/scc-dev
```

Características:
- Hot reload: Edita archivos, actualiza el navegador, ve los cambios
- Headers no-cache para actualizaciones inmediatas
- Mismos endpoints API que producción
- Tiempos de compilación más rápidos (sin embedding de activos)

### App de Escritorio (scc-desktop)

Aplicación de escritorio nativa usando el WebView de Wry/Tao.

```bash
cargo build --release --features desktop
./target/release/scc-desktop
```

Requiere la flag de feature `desktop` y las bibliotecas de sistema apropiadas.

### Herramienta de Benchmark (scc-benchmark)

Mide el rendimiento del servidor con pruebas de carga concurrente.

```bash
cargo build --release
./target/release/scc-benchmark

# Comandos:
scc-benchmark                      # Suite completa de benchmark
scc-benchmark quick                # Prueba rápida de conectividad
scc-benchmark endpoint /api/health # Probar endpoint específico

# Opciones:
scc-benchmark -h 192.168.1.100:9000  # Probar servidor remoto
scc-benchmark -n 1000 -c 50          # 1000 requests, 50 concurrentes
scc-benchmark -v                     # Salida detallada
```

### Cliente de Rendimiento (scc-perf-client)

Diagnósticos de rendimiento del lado del cliente con visualización de red.

```bash
cargo build --release
./target/release/scc-perf-client

# Comandos:
scc-perf-client test https://ejemplo.com      # Prueba completa de rendimiento
scc-perf-client measure https://ejemplo.com   # Desglose de tiempos
scc-perf-client trace ejemplo.com             # Visualización de traceroute

# Opciones:
scc-perf-client test -n 10 https://ejemplo.com  # 10 requests
scc-perf-client trace -m 20 ejemplo.com         # Máximo 20 saltos
```

## Estructura del Proyecto

```
southcitycomputer.com/
├── index.html              # Sitio web principal
├── css/
│   ├── style.css          # CSS fuente
│   └── style.min.css      # Minificado (embebido en binario)
├── js/
│   ├── main.js            # JavaScript fuente
│   └── main.min.js        # Minificado (embebido en binario)
├── images/                 # Imágenes WebP optimizadas
├── services/              # Páginas de servicios
├── app/                   # Página de descarga de app móvil
├── contact-handler/       # Servidor Rust
│   ├── src/
│   │   ├── main.rs        # Servidor de producción
│   │   ├── dev_server.rs  # Servidor de desarrollo
│   │   ├── desktop.rs     # App de escritorio
│   │   ├── benchmark.rs   # Benchmarks del servidor
│   │   └── perf_client.rs # Cliente de rendimiento
│   └── Cargo.toml
├── android-app/           # App Android WebView
├── PERFORMANCE_TESTING.md # Resultados de benchmarks
├── WHITEPAPER.md          # Profundización técnica
└── DEVELOPER.md           # Guía del desarrollador
```

## Flujo de Trabajo de Desarrollo

### 1. Usar Servidor de Desarrollo para Cambios de UI

```bash
# Terminal 1: Ejecutar servidor dev
cd contact-handler
cargo run --bin scc-dev

# Terminal 2: Editar archivos
# Los cambios en HTML/CSS/JS aparecen inmediatamente al actualizar el navegador
```

### 2. Compilar Binario de Producción

```bash
cd contact-handler
cargo build --release

# Probar que funciona
./target/release/scc-server &
curl http://localhost:9000/health
```

### 3. Ejecutar Benchmarks

```bash
# Verificación rápida
./target/release/scc-benchmark quick

# Suite completa
./target/release/scc-benchmark
```

## Desarrollo Guiado por Pruebas

Este proyecto sigue prácticas de TDD:

1. **Escribir pruebas que fallen primero** - Definir comportamiento esperado
2. **Escribir código mínimo** - Hacer que las pruebas pasen
3. **Refactorizar** - Limpiar mientras las pruebas se mantienen verdes

### Ejecutar Pruebas

```bash
cd contact-handler
cargo test

# Con salida
cargo test -- --nocapture

# Prueba específica
cargo test test_health_endpoint
```

### Categorías de Pruebas

- **Pruebas unitarias**: Comportamiento de funciones individuales
- **Pruebas de integración**: Comportamiento de endpoints API
- **Pruebas de benchmark**: Detección de regresión de rendimiento

## Endpoints API

| Endpoint | Método | Descripción |
|----------|--------|-------------|
| `/` | GET | Sitio web principal |
| `/health` | GET | Health check (JSON) |
| `/api/contact` | POST | Enviar formulario de contacto |
| `/view/contacts` | GET | Ver envíos (requiere auth) |
| `/*` | GET | Activos estáticos |

### API de Formulario de Contacto

```bash
curl -X POST http://localhost:9000/api/contact \
  -H "Content-Type: application/json" \
  -d '{"name":"Prueba","email":"test@ejemplo.com","message":"Hola"}'
```

Respuesta:
```json
{"success": true, "message": "Contacto guardado exitosamente", "id": "uuid"}
```

## Almacenamiento de Datos

Los envíos de contacto se almacenan en `contacts.csv`:

```csv
id,timestamp,name,email,phone,message,service
uuid,2026-01-15T10:30:00,Juan Pérez,juan@ejemplo.com,555-1234,Hola,reparación
```

### Procesamiento Desacoplado

El archivo CSV actúa como una cola. Los procesos externos pueden:

1. Leer nuevas entradas
2. Importar a base de datos
3. Enviar notificaciones por correo
4. Crear tickets de soporte
5. Limpiar entradas procesadas

Esto mantiene las operaciones sensibles (correo, base de datos) separadas del servidor web, reduciendo la superficie de ataque.

## Seguridad

### Actual

- Validación de entrada en todos los campos del formulario
- Almacenamiento CSV (no hay inyección SQL posible)
- Dependencias mínimas
- Sin sesiones de usuario (API sin estado)

### Planificado

- [ ] Almacenamiento CSV encriptado (AES-256)
- [ ] Hashing de contraseñas para cuentas admin (Argon2)
- [ ] Rate limiting en formulario de contacto
- [ ] Tokens CSRF para formularios

## Rendimiento

Documentado en [PERFORMANCE_TESTING.md](PERFORMANCE_TESTING.md) y [WHITEPAPER.md](WHITEPAPER.md).

### Métricas Clave (Enero 2026)

| Métrica | Valor |
|---------|-------|
| Throughput health check | 58,085 req/s |
| Throughput página principal | 26,795 req/s |
| Carga completa de página (5 activos) | 1.60ms |
| Latencia promedio | 36-382μs |
| Tamaño del binario | 19MB |
| Uso de memoria | ~1.1MB RSS |

### Resultados de Benchmark

```
Endpoint          Throughput    Latencia Prom    Transferencia
─────────────────────────────────────────────────────────────
Health Check      58,085 req/s      61μs       12.5 KB
Página Principal  26,795 req/s     122μs       1.99 MB
Hoja de Estilos   41,100 req/s      71μs       1.66 MB
JavaScript        45,406 req/s      66μs       1.24 MB
Logo (5KB)        39,287 req/s      65μs       270 KB
Storefront (128KB) 3,676 req/s     206μs       6.43 MB
─────────────────────────────────────────────────────────────
Calificación: ★★★★★ EXCELENTE (<100ms página completa)
```

### Resumen de Optimización

- Imágenes WebP: 78% más pequeñas que JPEG
- CSS/JS minificado: 24% más pequeño
- Activos residentes en memoria: Cero I/O de disco
- Lazy loading: 90% reducción de payload inicial
- Todos los activos embebidos en un solo binario

## Despliegue

### Binario Único

```bash
# Compilar
cargo build --release

# Desplegar
scp target/release/scc-server usuario@servidor:/opt/scc/

# Ejecutar
ssh usuario@servidor "/opt/scc/scc-server"
```

### Docker

```bash
docker build -t scc-server .
docker run -p 9000:9000 scc-server
```

### Servicio Systemd

```ini
[Unit]
Description=Servidor Web South City Computer
After=network.target

[Service]
Type=simple
ExecStart=/opt/scc/scc-server
Restart=always
User=www-data

[Install]
WantedBy=multi-user.target
```

## Minificación

CSS y JS se minifican usando clean-css y terser:

```bash
# Instalar herramientas
npm install

# Minificar
npx cleancss -o css/style.min.css css/style.css
npx terser js/main.js -o js/main.min.js --compress --mangle
```

Después de minificar, reconstruye el binario de producción para embeber los archivos actualizados.

## Licencia

Doble licencia:

- **GPLv3** para uso no comercial y código abierto - ver [LICENSE-GPL](LICENSE-GPL)
- **Licencia Comercial** para uso empresarial - ver [LICENSE-COMMERCIAL](LICENSE-COMMERCIAL)

Ver [LICENSE](LICENSE) para detalles sobre qué licencia aplica a tu caso de uso.

## Contribuir

1. Haz fork del repositorio
2. Crea una rama de feature
3. Escribe pruebas primero (TDD)
4. Haz tus cambios
5. Asegúrate de que todas las pruebas pasen
6. Envía un pull request

## Documentación

- [DEVELOPER.md](DEVELOPER.md) - Guía del desarrollador y arquitectura
- [PERFORMANCE_TESTING.md](PERFORMANCE_TESTING.md) - Metodología y resultados de benchmarks
- [WHITEPAPER.md](WHITEPAPER.md) - Profundización técnica en optimización de rendimiento
- [ROADMAP.md](ROADMAP.md) - Planes futuros e historias de usuario

## Contacto

South City Computer
- Sitio web: https://southcitycomputer.com
- Correo: info@southcitycomputer.com
