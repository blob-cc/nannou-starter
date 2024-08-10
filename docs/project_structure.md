# Project Structure

``` sh
nannou-template/
├── Cargo.toml
├── README.md
├── .gitignore
├── .github/
│   └── workflows/
│       └── ci.yml
├── assets/
│   ├── shaders/
│   │   ├── basic.frag
│   │   └── basic.vert
│   ├── fonts/
│   │   └── OpenSans-Regular.ttf
│   └── textures/
│       └── texture.png
├── examples/
│   ├── animated_shapes.rs
│   ├── custom_shader.rs
│   ├── simple_shape.rs
│   └── texture_example.rs
├── src/
│   ├── main.rs
│   ├── model.rs
│   ├── utils/
│   │   ├── color.rs
│   │   ├── geometry.rs
│   │   └── shader.rs
└── tests/
    └── integration_test.rs
```
