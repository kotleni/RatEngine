#version 330 core

in vec3 fragPos;
in vec3 fragNormal;
in vec3 viewDir;
in float oTime;

out vec4 fragColor;

void main() {
    vec3 lightColor = vec3(1.0, 1.0, 0.9);

    float colorOffset = 0.05 * sin(oTime + fragPos.x + fragPos.z * 2.5);

    vec3 waterColor = vec3(0.2 + colorOffset, 0.1 + color_offset, 0.6 + color_offset);
    float shininess = 0;

    // Calculate the direction vector from the fragment to the light source
    vec3 lightDir = normalize(vec3(0.0, 1.0, 0.0)); // Adjust the light direction as needed

    // Calculate ambient lighting
    vec3 ambient = vec3(0.1) * waterColor;

    // Calculate diffuse lighting
    float diff = max(dot(fragNormal, lightDir), 0.0);
    vec3 diffuse = diff * lightColor * waterColor;

    // Calculate specular lighting
    vec3 reflectDir = reflect(-lightDir, fragNormal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = spec * lightColor;

    // Combine ambient, diffuse, and specular components
    vec3 result = (ambient + diffuse + specular) * waterColor;

    // Set the fragment color
    fragColor = vec4(result, 0.8);
}
