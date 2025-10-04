#!/bin/bash
# Scripts de formateo y análisis para el proyecto Rust

# Función para mostrar ayuda
show_help() {
    echo "Scripts de desarrollo para el proyecto GPX Extractor"
    echo ""
    echo "Uso: ./dev.sh [comando]"
    echo ""
    echo "Comandos disponibles:"
    echo "  fmt        - Formatear código con rustfmt"
    echo "  fmt-nightly - Formatear con rustfmt nightly (más opciones)"
    echo "  clippy     - Ejecutar clippy para análisis de código"
    echo "  clippy-fix - Ejecutar clippy con auto-fix"
    echo "  check      - Verificar compilación"
    echo "  test       - Ejecutar tests"
    echo "  audit      - Verificar vulnerabilidades"
    echo "  clean      - Limpiar artifacts de compilación"
    echo "  all        - Ejecutar todas las verificaciones"
    echo "  help       - Mostrar esta ayuda"
}

# Formateo con rustfmt estable
fmt() {
    echo "🎨 Formateando código con rustfmt..."
    cargo fmt
}

# Formateo con rustfmt nightly
fmt_nightly() {
    echo "🎨 Formateando código con rustfmt nightly..."
    if command -v rustup &> /dev/null; then
        if rustup toolchain list | grep -q nightly; then
            cargo +nightly fmt --config-path rustfmt-nightly.toml
        else
            echo "❌ Rustfmt nightly no está instalado."
            echo "📥 Instala con: rustup toolchain install nightly && rustup component add rustfmt --toolchain nightly"
        fi
    else
        echo "❌ Rustup no está disponible"
    fi
}

# Análisis con clippy (herramienta externa, no integrada en el editor)
clippy() {
    echo "🔍 Ejecutando clippy..."
    echo "💡 Nota: Clippy se ejecuta separadamente del editor para análisis avanzado"
    cargo clippy -- -W clippy::all -W dead_code -W unused_variables
}

# Clippy con auto-fix
clippy_fix() {
    echo "🔧 Ejecutando clippy con auto-fix..."
    cargo clippy --fix --allow-dirty -- -W clippy::all -W dead_code -W unused_variables
}

# Verificar compilación
check() {
    echo "✅ Verificando compilación..."
    cargo check
}

# Ejecutar tests
test() {
    echo "🧪 Ejecutando tests..."
    cargo test
}

# Auditoría de seguridad
audit() {
    echo "🔒 Verificando vulnerabilidades..."
    if command -v cargo-audit &> /dev/null; then
        cargo audit
    else
        echo "❌ cargo-audit no está instalado."
        echo "📥 Instala con: cargo install cargo-audit"
    fi
}

# Limpiar artifacts
clean() {
    echo "🧹 Limpiando artifacts..."
    cargo clean
}

# Ejecutar todas las verificaciones
all() {
    echo "🚀 Ejecutando todas las verificaciones..."
    fmt
    clippy
    check
    test
    audit
    echo "✨ ¡Todas las verificaciones completadas!"
}

# Procesar argumentos
case "${1:-help}" in
    fmt)
        fmt
        ;;
    fmt-nightly)
        fmt_nightly
        ;;
    clippy)
        clippy
        ;;
    clippy-fix)
        clippy_fix
        ;;
    check)
        check
        ;;
    test)
        test
        ;;
    audit)
        audit
        ;;
    clean)
        clean
        ;;
    all)
        all
        ;;
    help|*)
        show_help
        ;;
esac