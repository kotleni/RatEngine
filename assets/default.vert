#version 330 core
layout(location = 0) in vec3 position;

uniform mat4 model; // Model matrix
uniform mat4 view;  // View matrix
uniform mat4 projection; // Projection matrix

void main() {
    // Apply transformations to the model's vertices
    gl_Position = projection * view * model * vec4(position, 1.0);
}
