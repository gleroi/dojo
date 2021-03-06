#version 330

layout (location = 0) in vec3 Position;

uniform mat4 gWorld;

out vec4 Color;

void main()
{
    Color = vec4(clamp(Position, 0.0, 1.0), 0.0);
    gl_Position = gWorld * vec4(Position, 1.0);
}