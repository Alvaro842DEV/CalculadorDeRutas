# Descripción

Algoritmo moderno que busca ser más eficiente y rápido que A* y Dijkstra en grafos grandes y complejos.

# Características y funciones

- Búsqueda bidireccional: permite realizar la búsqueda simultáneamente desde el punto de inicio y el punto final, reduciendo el espacio de búsqueda.
- Heurística adaptativa: la heurística se ajusta dinámicamente según la distancia recorrida y regiones exploradas, haciendo que sea más inteligente según las características del grafo.
- Partición en regiones: el grafo se particiona en regiones para mejora la búsqueda en grafos grandes, esto reduce el tiempo de búsqueda y optimiza el uso de memoria.

# Instalación

Pasos para descargar el proyecto y ejecutar el algoritmo.

## Requisitos

Debes tener instalado **Rust** y **Cargo**

Cargo es el administrador de paquetes de Rust, por lo que al instalar Rust ya tendrás instalado Cargo.

1. - Descarga Rust desde la [web oficial](https://www.rust-lang.org/tools/install)
2. - Después de descargar, comprueba que se ha instalado correctamente:
```bash
rustc --version
cargo --version
```

## Descarga

Para descargar el proyecto debes utilizar esta serie de comandos:

```bash
# Clonar el proyecto
git clone https://github.com/Alvaro842DEV/CalculadorDeRutas.git

# Ir a la carpeta del proyecto
cd CalculadorDeRutas

# Ejecutar el proyecto
cargo run

# En caso de que solo quieras compilar sin ejecutar
cargo build
```

### Licencia
Este proyecto está bajo la **licencia GPL v3**, tal como se especifica en el archivo `LICENSE`. Si deseas saber más sobre esta licencia y cómo cumplir con sus términos, puedes consultar el archivo [aquí](https://github.com/Alvaro842DEV/CalculadorDeRutas/blob/main/LICENSE.md).

### Contribuciones

Antes de nada, ¡gracias por contribuir! Si deas enviar una **pull request**, debes seguir estas reglas:

1. Código estructurado
- Asegúrate que tu código es legible.
- Si puedes o es necesario, añade comentarios a tu código.

2. Actualiza el [CHANGELOG.md](https://github.com/Alvaro842DEV/CalculadorDeRutas/blob/main/CHANGELOG.md)
- Asegurate de poner la actualización y todo lo que añadiste/corregiste/eliminaste.
- Manten siempre el formato
- El formato de actualización es asi (ejemplo):
  - 24 **Indica el año en el que se lanza la actualización**
  - .0 **Indica en que versión estable estamos**
  - -b1 **Indica en que versión beta estamos**

3. Haz una pull request limpia
- Pon un título que tenga que ver con tus cambios.
- Pon una descripción sobre lo que hiciste en la pull request.

4. No actualizes el [README.md](https://github.com/Alvaro842DEV/CalculadorDeRutas/blob/main/README.md)
- **Nunca** actualizes el README al hacer un pull request, yo me encargare de actualizarlo si lo veo necesario.
- Si crees que es necesario actualizarlo, puedes pedirme que lo actualize desde la descripción de tu pull request.
