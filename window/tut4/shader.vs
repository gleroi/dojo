#version 330

layout (location = 0) in vec3 Position;

uniform float gScale;

out vec4 Color;

void main()
{
    gl_Position = vec4(gScale * Position.x, gScale * Position.y, Position.z, 1.0);
    Color = vec4(clamp(Position, 0.0, 1.0), 1.0);
}