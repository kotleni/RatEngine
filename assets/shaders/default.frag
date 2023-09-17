#version 330 core

in vec3 fragPos;
in vec3 fragNormal;
in vec2 fragTexCoord;
in vec3 lightDir;

uniform vec3 objectColor;
uniform vec3 viewPos;
uniform sampler2D ourTexture;

out vec4 color;

void main() {
    vec3 lightColor = vec3(1.0, 1.0, 0.9);

    // ambient
    float ambientStrength = 0.2;
    vec3 ambient = ambientStrength * lightColor;

    // diffuse
    vec3 norm = normalize(fragNormal);
    vec3 lightDirNormalized = normalize(lightDir); // Normalize the light direction
    float diff = max(dot(norm, lightDirNormalized), 0.0);
    vec3 diffuse = diff * lightColor;

    // specular
    float specularStrength = 0.5;
    vec3 viewDir = normalize(viewPos - fragPos);
    vec3 reflectDir = reflect(-lightDirNormalized, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.1), 32);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = (ambient + diffuse + specular) * objectColor;
    // color = vec4(result, 1.0);
    color = texture(ourTexture, fragTexCoord) * vec4(result, 1.0);
}
