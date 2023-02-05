#version 130

uniform sampler2D sampler;

uniform int totalColumns;
uniform int totalRows;

uniform int currentColumn;
uniform int currentRow;

in vec2 tex_coords;

void main() {
    vec2 txtSize = textureSize(sampler, 0);
    vec2 onePixel = vec2(3.0,3.0) / txtSize;
    vec2 spriteSize = vec2(1.0 / totalColumns - 2.0 * onePixel.x, 1.0 / totalRows - 2.0 * onePixel.y);
    vec2 spriteOffset = vec2(float(currentColumn) / totalColumns + onePixel.x, float(currentRow) / totalRows + onePixel.y);
    vec2 textureCoordinate = vec2(tex_coords.x * spriteSize.x + spriteOffset.x, tex_coords.y * spriteSize.y + spriteOffset.y);
    gl_FragColor = texture2D(sampler, textureCoordinate);
}
