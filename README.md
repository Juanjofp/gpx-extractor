# GPX Extractor

Un extractor de datos GPX escrito en Rust.

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
