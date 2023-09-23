#version 330 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

out vec3 fragPos;
out vec3 fragNormal;
out vec3 viewDir;
out float oTime;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 cameraPos;
uniform float time; // Add a uniform for time

void main() {
    // Calculate fragment position and normal in world space
    fragPos = vec3(model * vec4(position, 1.0));
    fragNormal = mat3(transpose(inverse(model))) * normal;

    // Calculate the direction vector from the fragment to the camera
    viewDir = cameraPos - fragPos;

    // Add a wave-like effect by displacing the vertices vertically
    float waveAmplitude = 0.1; // Adjust the amplitude as needed
    float waveFrequency = 1.0; // Adjust the frequency as needed
    vec3 displacedPosition = position;
    if (position.y > 0.0)
        displacedPosition.y += waveAmplitude * sin(time * waveFrequency + position.x + position.z);

    // Calculate the gl_Position
    gl_Position = projection * view * vec4(displacedPosition, 1.0);
    oTime = time;
}
