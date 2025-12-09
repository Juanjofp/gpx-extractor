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
use std::convert::TryFrom;

// Método 1: Usando try_from_str (método específico)
let gpx = match Gpx::try_from_str(xml_content) {
    Ok(gpx) => gpx,
    Err(e) => {
        eprintln!("Error parsing GPX: {}", e);
        return;
    }
};

// Método 2: Usando TryFrom trait (más idiomático en Rust)
let gpx = match Gpx::try_from(xml_content) {
    Ok(gpx) => gpx,
    Err(e) => {
        eprintln!("Error parsing GPX: {}", e);
        return;
    }
};

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
let reparsed_gpx = Gpx::try_from_str(&xml_string).expect("Error en roundtrip");
assert_eq!(gpx.total_points(), reparsed_gpx.total_points());
```

### 🚀 Ejecutar Ejemplo

```bash
# Ejecutar demo de conversión GPX→XML
cargo run --example gpx_to_xml_demo
```

## ⚠️ Manejo de Errores

La librería proporciona dos métodos idiomáticos para manejo de errores explícito:

### Método 1: `try_from_str` (método específico)

```rust
match Gpx::try_from_str(xml_content) {
    Ok(gpx) => {
        println!("GPX cargado correctamente con {} tracks", gpx.tracks.len());
        // Procesar el GPX...
    },
    Err(e) => {
        eprintln!("Error parsing GPX: {}", e);
        // Manejar el error apropiadamente
    }
}
```

### Método 2: `TryFrom` trait (más idiomático)

```rust
use std::convert::TryFrom;

match Gpx::try_from(xml_content) {
    Ok(gpx) => {
        println!("GPX cargado correctamente con {} tracks", gpx.tracks.len());
        // Procesar el GPX...
    },
    Err(e) => {
        eprintln!("Error parsing GPX: {}", e);
        // Manejar el error apropiadamente
    }
}

// También funciona con el operador ?
fn load_gpx(xml: &str) -> Result<Gpx, quick_xml::DeError> {
    let gpx = Gpx::try_from(xml)?;
    Ok(gpx)
}
```

### Beneficios del Manejo Explícito de Errores

- **🔒 Seguridad:** No hay métodos que silenciosamente devuelvan estructuras vacías
- **🐛 Depuración:** Los errores contienen información específica sobre qué falló
- **🎯 Precisión:** El llamador siempre sabe si el parsing fue exitoso o no
- **📊 Monitoreo:** Puedes registrar, contar y manejar errores de parsing apropiadamente

```rust
// ❌ Antes: No sabías si el GPX estaba realmente vacío o hubo un error
let gpx = Gpx::from(possibly_invalid_xml);
if gpx.is_empty() {
    // ¿Era un GPX vacío válido o un error de parsing?
}

// ✅ Ahora: Manejo explícito y claro
use std::convert::TryFrom;

match Gpx::try_from(xml_content) {
    Ok(gpx) if gpx.is_empty() => println!("GPX válido pero vacío"),
    Ok(gpx) => println!("GPX cargado con {} tracks", gpx.tracks.len()),
    Err(e) => {
        log::error!("Error parsing GPX: {}", e);
        // Manejar error apropiadamente
    }
}
```

### 💡 Mejores Prácticas

#### Cuándo usar cada método:

- **`try_from_str()`**: Cuando quieres ser explícito sobre el parsing de strings
- **`TryFrom` trait**: Más idiomático, funciona bien con genéricos y permite usar el operador `?`

```rust
use std::convert::TryFrom;

// ✅ Excelente para manejo de errores con ?
fn process_gpx_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let gpx = Gpx::try_from(content.as_str())?;

    println!("Procesando GPX con {} tracks", gpx.tracks.len());
    Ok(())
}

// ✅ Funciona bien con código genérico
fn parse_from_string<T>(s: &str) -> Result<T, T::Error>
where
    T: TryFrom<&str>,
{
    T::try_from(s)
}

let gpx: Gpx = parse_from_string(xml_content)?;
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
