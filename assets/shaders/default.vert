#version 330 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 texCoord;

out vec3 fragPos;    // Store fragment position
out vec3 fragNormal;
out vec2 fragTexCoord;
out vec3 lightDir;    // Store light direction

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec3 lightPos; // Light position

void main() {
    vec2 textureScale = vec2(1.0, 1.0);

    vec4 fragPos4 = model * vec4(position, 1.0);
    fragPos = fragPos4.xyz; // Store fragment position in world coordinates
    fragNormal = mat3(transpose(inverse(model))) * normal;

    // Calculate light direction from fragment to light
    lightDir = lightPos - fragPos;

    gl_Position = projection * view * fragPos4;
    fragTexCoord = vec2(texCoord.x * textureScale.x, texCoord.y * textureScale.y);
}
