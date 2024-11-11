# 🌌 Solar System 3D Renderer

Este proyecto es una simulación interactiva en 3D de nuestro Sistema Solar, implementada en Rust. Utiliza gráficos generados manualmente, cálculos matemáticos para órbitas planetarias y modelos OBJ para renderizar los planetas y una nave espacial en tercera persona. También incluye un skybox para mostrar estrellas en el horizonte.

## 🚀 Tecnologías Utilizadas

- **Rust**: Lenguaje de programación utilizado para el desarrollo.
- **Minifb**: Librería para manejar la ventana y capturar eventos de teclado.
- **Nalgebra GLM**: Librería para operaciones matemáticas en gráficos 3D.
- **OBJ Loader**: Módulo personalizado para cargar modelos OBJ.
- **Shaders Personalizados**: Sombras y efectos de iluminación implementados manualmente.

## 📂 Estructura del Proyecto

```
CODIGO
.
├── assets/
│   ├── models/
│   │   ├── sun.obj
│   │   ├── planet.obj
│   │   └── naveEspacial1.obj
│   └── textures/
│       └── skybox.png
├── src/
│   ├── main.rs
│   ├── camera.rs
│   ├── color.rs
│   ├── fragment.rs
│   ├── framebuffer.rs
│   ├── obj.rs
│   ├── shaders.rs
│   ├── triangle.rs
│   └── vertex.rs
└── Cargo.toml
CODIGO
```

## 🌍 Características

### Sistema Solar Completo:

- Simulación de planetas: Mercurio, Venus, Tierra (con su Luna), Marte, Júpiter y Saturno (con anillos).
- Cálculo de órbitas utilizando funciones trigonométricas.
- Distancias escaladas usando la constante `DISTANCE_SCALE` para ajustar el tamaño del Sistema Solar.

### Nave Espacial en Tercera Persona:

- Una nave espacial sigue a la cámara, simulando una vista en tercera persona.
- El modelo de la nave se carga desde un archivo OBJ.
- La nave está siempre posicionada frente a la cámara y se renderiza con un shader específico.

### Skybox con Estrellas:

- Se utiliza una textura para crear un skybox, dando la ilusión de un espacio lleno de estrellas.
- Implementado con vértices manuales para las seis caras del cubo del skybox.

### Cálculo de FPS:

- El programa muestra los FPS en tiempo real en el título de la ventana.

### Interacción con la Cámara:

- La cámara se puede mover usando las teclas:
  - `W`, `S`: Rotar hacia arriba y abajo.
  - `A`, `D`: Rotar hacia la izquierda y derecha.
  - `Q`, `E`: Mover la cámara hacia arriba y abajo.
  - `Up`, `Down`: Acercar y alejar el zoom.
  - `Left`, `Right`: Orbitar alrededor del centro.

## 🛠️ Instalación y Ejecución

### Requisitos Previos:

- Tener instalado Rust y Cargo.

### Clonar el Proyecto:

```
git clone https://github.com/Diegoval-Dev/SpaceTraver-GC
cd SpaceTraver-GC
```

### Compilar y Ejecutar:

```
cargo run --release

```

### Solución de Problemas:

- Si ves mensajes de advertencia (warnings), puedes solucionarlos automáticamente con:

```

cargo fix --bin <NOMBRE_DEL_BINARIO>

```

## 🔍 Detalles Técnicos

- **Shaders Personalizados**: Cada planeta tiene su propio shader para efectos visuales específicos (rocosos, gaseosos, con anillos, etc.).
- **Órbitas Planetarias**: Utiliza funciones seno y coseno para calcular las posiciones de los planetas en tiempo real.
- **Modelos OBJ**: Los modelos se cargan desde archivos OBJ utilizando un cargador personalizado.
- **Skybox**: Implementado con una textura mapeada a las seis caras del cubo del skybox.

## 💻 Controles del Usuario

| Tecla       | Acción                          |
|-------------|---------------------------------|
| `W`, `S`    | Rotar la cámara verticalmente   |
| `A`, `D`    | Rotar la cámara horizontalmente |
| `Q`, `E`    | Mover la cámara arriba/abajo    |
| `Up`, `Down`| Zoom de la cámara               |
| `Left`, `Right` | Orbitar alrededor del centro |
| `Escape`    | Salir del programa              |

## 📦 Archivos Importantes

- `main.rs`: Controlador principal del programa.
- `camera.rs`: Implementa la lógica de la cámara.
- `shaders.rs`: Contiene los shaders personalizados para los planetas, nave espacial y skybox.
- `framebuffer.rs`: Módulo para manejar el framebuffer y dibujar píxeles.
- `obj.rs`: Cargador de modelos OBJ.
- `assets/textures/skybox.png`: Textura utilizada para el skybox.

## 🌟 Futuras Mejoras

- Agregar más planetas y sus lunas.
- Mejorar los shaders para simular atmósferas y sombras.
- Añadir efectos de partículas (e.g., meteoros o cometas).
- Implementar colisiones o eventos dinámicos en el Sistema Solar.

## 👨‍💻 Contribuciones

Las contribuciones son bienvenidas. Puedes abrir un issue o hacer un pull request con tus sugerencias o mejoras.

## 📝 Licencia

Este proyecto está bajo la Licencia MIT. Puedes ver los detalles en el archivo LICENSE.