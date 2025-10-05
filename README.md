# GPX Extractor

Un extractor de datos GPX escrito en Rust.

## 🗺️ Esquema de Estructura GPX

Este proyecto maneja archivos GPX (GPS Exchange Format) que se parsan desde XML. La estructura jerárquica del GPX es la siguiente:

```
GPX
├── Tracks (trk) - Rutas grabadas
│   ├── Name - Nombre del track
│   └── Track Segments (trkseg) - Segmentos continuos
│       └── Track Points (trkpt) - Puntos individuales
│           ├── @lat - Latitud
│           ├── @lon - Longitud
│           ├── ele - Elevación (opcional)
│           └── time - Timestamp (opcional)
└── Waypoints (wpt) - Puntos de interés
    ├── @lat - Latitud
    ├── @lon - Longitud
    ├── name - Nombre del waypoint
    ├── ele - Elevación (opcional)
    └── time - Timestamp (opcional)
```

### 🔧 Funcionalidades Implementadas

#### Parsing desde XML

- ✅ **Deserialización automática** usando `serde` y `quick-xml`
- ✅ **Manejo de errores** en caso de XML inválido
- ✅ **Soporte completo** para tracks, segmentos, puntos y waypoints

#### Análisis de Datos

- ✅ **Cálculo de distancias** usando fórmula Haversine
- ✅ **Estadísticas de elevación** (mín, máx, ganancia)
- ✅ **Conteo de elementos** (tracks, segmentos, puntos, waypoints)
- ✅ **Resumen automático** de contenido del GPX

#### Operaciones de Construcción

- ✅ **Creación programática** de estructuras GPX
- ✅ **Adición de tracks y waypoints**
- ✅ **Validación** de contenido vacío
- ✅ **Nombres automáticos** para elementos sin nombre

#### Serialización a XML

- ✅ **Conversión GPX→XML** usando `quick-xml` y `serde`
- ✅ **Múltiples interfaces** (`to_xml()`, `Display`, `Into<String>`)
- ✅ **Guardado en archivo** con manejo de errores
- ✅ **Roundtrip completo** (XML→GPX→XML)
- ✅ **Manejo inteligente** de campos opcionales

#### Utilidades Geográficas

- ✅ **Distancia entre puntos** (algoritmo Haversine)
- ✅ **Agregación de distancias** por segmento y track
- ✅ **Análisis de elevación** con rangos y ganancias

### 📊 Ejemplo de Uso

```rust
use gpx_extractor::{Gpx, Track, TrackSegment, Point, Waypoint};

// Crear desde XML
let gpx = Gpx::from(xml_content);

// Obtener estadísticas
let stats = gpx.statistics();
println!("Distancia total: {:.2} km", stats.total_distance_km);
println!("Puntos: {}", stats.total_points);

// Crear programáticamente
let mut gpx = Gpx::new();
let mut track = Track::with_name("Mi Ruta".to_string());
let segment = TrackSegment::with_points(vec![
    Point::with_elevation(40.7128, -74.0060, 10.0),
    Point::with_elevation(40.7589, -73.9851, 15.0),
]);
track.add_segment(segment);
gpx.add_track(track);
gpx.add_waypoint(Waypoint::with_name(40.7128, -74.0060, "NYC".to_string()));

// Convertir a XML (múltiples formas)
let xml_string = gpx.to_xml();           // Método directo
let xml_display = format!("{}", gpx);    // Usando Display trait
let xml_into: String = gpx.into();       // Usando Into<String>

// Guardar en archivo
gpx.save_to_file("mi_ruta.gpx").expect("Error guardando archivo");

// Roundtrip: XML → GPX → XML
let reparsed_gpx: Gpx = Gpx::from(&xml_string);
assert_eq!(gpx.total_points(), reparsed_gpx.total_points());
```

### 🚀 Ejecutar Ejemplo

```bash
# Ejecutar demo de conversión GPX→XML
cargo run --example gpx_to_xml_demo
```

## 📋 Configuración de Desarrollo

Este proyecto está configurado con herramientas de calidad de código para mantener un estilo consistente y detectar problemas automáticamente.

### 🛠️ Herramientas Configuradas

- **rustfmt** - Formateo automático de código
- **Clippy** - Linter para detectar problemas y mejores prácticas
- **cargo-audit** - Verificación de vulnerabilidades de seguridad

### 🎨 Formateo de Código

#### Configuración Básica (Estable)

```bash
# Formatear todo el proyecto
cargo fmt

# O usar el script de desarrollo
./dev.sh fmt
```

#### Configuración Avanzada (Nightly)

Para características avanzadas de formateo, instala rustfmt nightly:

```bash
# Instalar rustfmt nightly
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly

# Usar formateo avanzado
./dev.sh fmt-nightly
```

### 🔍 Análisis de Código

```bash
# Ejecutar clippy
cargo clippy -- -W clippy::all

# O usar el script
./dev.sh clippy

# Auto-fix automático (cuando sea posible)
./dev.sh clippy-fix
```

### 🔒 Auditoría de Seguridad

```bash
# Verificar vulnerabilidades
cargo audit

# O usar el script
./dev.sh audit
```

### 🚀 Script de Desarrollo

El archivo `dev.sh` incluye comandos útiles para desarrollo:

```bash
./dev.sh help           # Mostrar ayuda
./dev.sh fmt            # Formatear código
./dev.sh clippy         # Ejecutar clippy
./dev.sh check          # Verificar compilación
./dev.sh test           # Ejecutar tests
./dev.sh audit          # Verificar vulnerabilidades
./dev.sh all            # Ejecutar todas las verificaciones
```

### 📁 Archivos de Configuración

- `rustfmt.toml` - Configuración de formateo para versión estable
- `rustfmt-nightly.toml` - Configuración avanzada para nightly
- `clippy.toml` - Configuración de reglas de Clippy
- `.vscode/settings.json` - Configuración de VS Code para el proyecto

### ⚙️ Configuración en VS Code

El proyecto incluye configuración simple y efectiva para VS Code:

- ✅ Formateo automático al guardar
- ✅ Detección de errores y warnings en tiempo real
- ✅ Configuración minimalista y fácil de replicar
- ✅ Análisis básico de código integrado

## 🏗️ Compilación y Ejecución

```bash
# Compilar
cargo build

# Ejecutar
cargo run <archivo.gpx>

# Ejecutar tests
cargo test
```

## 📝 Estilo de Código

Este proyecto sigue las convenciones estándar de Rust con algunas personalizaciones:

- **Ancho máximo de línea**: 100 caracteres
- **Indentación**: 4 espacios
- **Imports**: Organizados automáticamente
- **Reordenamiento**: Los imports se reorganizan automáticamente

Para mantener la consistencia, por favor ejecuta `cargo fmt` antes de hacer commits.
