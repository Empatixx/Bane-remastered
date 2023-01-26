#version 130

uniform sampler2D sampler;

uniform int totalColumns;
uniform int totalRows;

uniform int currentColumn;
uniform int currentRow;

in vec2 tex_coords;

void main() {
    vec2 spriteSize = vec2(1.0 / totalColumns, 1.0 / totalRows);
    vec2 spriteOffset = vec2(float(currentColumn) / totalColumns, float(currentRow) / totalRows);
    vec2 textureCoordinate = vec2(tex_coords.x * spriteSize.x + spriteOffset.x,tex_coords.y * spriteSize.y + spriteOffset.y);
    gl_FragColor = texture2D(sampler, textureCoordinate);
}
