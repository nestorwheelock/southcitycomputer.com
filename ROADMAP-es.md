# Hoja de Ruta e Historias de Usuario

## Visión

Proporcionar una alternativa de código abierto a WordPress que demuestre cuán simple, rápido y mantenible puede ser un sitio web de pequeña empresa. Compartir conocimiento sobre optimización de rendimiento y arquitectura desacoplada.

---

## Historias de Usuario

### Visitantes del Sitio Web

**S-001: Carga de Página Rápida**
> Como visitante, quiero que el sitio web cargue instantáneamente para poder encontrar información rápidamente sin esperar.

Criterios de Aceptación:
- [x] Carga de página completa bajo 100ms (localhost)
- [x] Contenido above-the-fold bajo 50ms
- [x] Todas las imágenes optimizadas (WebP)
- [x] Lazy loading para imágenes below-fold

**S-002: Formulario de Contacto**
> Como cliente potencial, quiero enviar un formulario de contacto para poder solicitar servicios o hacer preguntas.

Criterios de Aceptación:
- [x] El formulario valida entrada antes de enviar
- [x] Mensajes claros de éxito/error
- [x] Datos del formulario guardados de forma segura
- [ ] Confirmación por correo enviada (futuro)

**S-003: Experiencia Móvil**
> Como usuario móvil, quiero que el sitio web funcione bien en mi teléfono para poder acceder desde cualquier lugar.

Criterios de Aceptación:
- [x] Diseño responsive
- [x] Navegación amigable al tacto
- [x] Carga rápida en redes móviles
- [x] App Android disponible

### Dueño del Negocio

**S-004: Ver Envíos de Contacto**
> Como dueño del negocio, quiero ver los envíos del formulario de contacto para poder responder a las consultas de clientes.

Criterios de Aceptación:
- [x] Acceso autenticado a envíos
- [x] Ver todos los envíos con timestamps
- [ ] Exportar envíos (futuro)
- [ ] Marcar como respondido (futuro)

**S-005: Despliegue Fácil**
> Como administrador del sitio, quiero un despliegue simple para poder actualizar el sitio rápidamente.

Criterios de Aceptación:
- [x] Despliegue de binario único
- [x] Sin dependencias externas
- [x] Soporte Docker
- [x] Configuración de servicio systemd

### Desarrolladores

**S-006: Modo Desarrollo**
> Como desarrollador, quiero un servidor de desarrollo que muestre cambios instantáneamente para poder iterar rápidamente.

Criterios de Aceptación:
- [x] Servidor dev lee archivos del disco
- [x] Sin recompilación para cambios HTML/CSS/JS
- [x] Misma API que producción
- [x] Documentación clara

**S-007: Pruebas de Rendimiento**
> Como desarrollador, quiero herramientas de benchmarking para poder medir y optimizar el servidor.

Criterios de Aceptación:
- [x] Herramienta de benchmark del lado del servidor
- [x] Medición de rendimiento del lado del cliente
- [x] Visualización de traceroute
- [x] Resultados de benchmark documentados

---

## Hoja de Ruta

### v0.2.0 - Release Actual

**Completado:**
- [x] Binario Rust monolítico con activos embebidos
- [x] Optimización de imágenes WebP (78% reducción)
- [x] Minificación CSS/JS
- [x] Lazy loading con precarga al scroll
- [x] API de formulario de contacto con almacenamiento CSV
- [x] Servidor de desarrollo (basado en disco)
- [x] Wrapper de app de escritorio
- [x] Herramientas de benchmark
- [x] Documentación de rendimiento
- [x] Doble licencia (GPLv3 / Comercial)

### v0.3.0 - Endurecimiento de Seguridad

**Planificado:**
- [ ] Almacenamiento CSV encriptado (AES-256-GCM)
- [ ] Hashing de contraseñas para cuentas admin (Argon2id)
- [ ] Rate limiting en formulario de contacto
- [ ] Tokens CSRF para formularios
- [ ] Auditoría y documentación de seguridad

**Historia de Usuario: Almacenamiento de Datos Encriptado**
> Como dueño de negocio, quiero que los datos del formulario de contacto estén encriptados en reposo para que la información del cliente esté protegida si el servidor es comprometido.

Notas de Implementación:
- Usar crate `ring` o `aes-gcm` para encriptación
- Clave almacenada en variable de entorno o gestor de secretos
- Encriptar cada fila individualmente para operaciones de append
- Desencriptar al leer para vista de admin

**Historia de Usuario: Autenticación Segura**
> Como admin, quiero que mi contraseña esté hasheada de forma segura para que mis credenciales estén protegidas.

Notas de Implementación:
- Usar crate `argon2` para hashing de contraseñas
- Almacenar contraseñas hasheadas en archivo de configuración separado
- Implementar bloqueo de cuenta después de intentos fallidos

### v0.4.0 - Monitoreo Mejorado

**Planificado:**
- [ ] Logging estructurado (formato JSON)
- [ ] Endpoint de métricas Prometheus
- [ ] Health check con estado detallado
- [ ] Trazado de requests
- [ ] CI de regresión de rendimiento

### v0.5.0 - Integración de Email

**Planificado:**
- [ ] Envío de email SMTP para notificaciones de contacto
- [ ] Confirmación por email al que envía el formulario
- [ ] Plantillas de email (HTML + texto plano)
- [ ] Configuraciones de email configurables

**Historia de Usuario: Notificaciones de Contacto**
> Como dueño de negocio, quiero recibir notificaciones por email cuando alguien envía el formulario de contacto para poder responder rápidamente.

Notas de Implementación:
- Usar crate `lettre` para SMTP
- Encolar emails en archivo separado (desacoplado del request web)
- Worker de background procesa cola de email
- Configuraciones SMTP configurables vía environment

### v0.6.0 - Dashboard de Admin

**Planificado:**
- [ ] Interfaz de administración basada en web
- [ ] Gestión de envíos de contacto
- [ ] Analíticas básicas (vistas de página)
- [ ] Edición de contenido (limitada)

### Consideraciones Futuras

**Rendimiento:**
- Soporte HTTP/3 QUIC
- Despliegue edge (Cloudflare Workers, Fly.io)
- Service Worker para soporte offline
- Srcset de imágenes para imágenes responsive

**Funcionalidades:**
- Gestión de contenido multi-idioma
- Sección de blog/noticias
- Programación de citas
- Integración de chat en vivo

**Infraestructura:**
- Configuraciones de despliegue Kubernetes
- Configuración de auto-scaling
- Integración CDN
- Automatización de backups

---

## Deuda Técnica

Items a abordar en releases futuros:

1. **Cobertura de Pruebas**
   - Añadir pruebas de integración para todos los endpoints
   - Añadir pruebas basadas en propiedades para validación
   - Configurar CI/CD con reporte de cobertura

2. **Manejo de Errores**
   - Implementar tipos de error personalizados
   - Mejores mensajes de error para usuarios
   - Logging de errores estructurado

3. **Configuración**
   - Configuración basada en environment
   - Soporte de archivo de configuración (TOML/YAML)
   - Feature flags

4. **Documentación**
   - Documentación API (OpenAPI/Swagger)
   - Registros de decisiones de arquitectura
   - Playbooks de despliegue

---

## Contribuir

¿Quieres trabajar en algo de la hoja de ruta?

1. Verificar si hay un issue abierto para la funcionalidad
2. Comentar en el issue para reclamarlo
3. Seguir enfoque TDD (pruebas primero)
4. Enviar PR referenciando el issue

Para nuevas funcionalidades no en la hoja de ruta:
1. Abrir un issue para discutir
2. Obtener aprobación antes de empezar
3. Seguir patrones y estilo existentes

---

## Historial de Versiones

| Versión | Fecha | Destacados |
|---------|-------|------------|
| v0.1.0 | 2026-01-13 | Release inicial, nginx + Rust API |
| v0.2.0 | 2026-01-15 | Binario monolítico, WebP, benchmarks |

---

*Última Actualización: Enero 2026*
