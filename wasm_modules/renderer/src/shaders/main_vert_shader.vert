attribute vec4 aVertexPosition;
attribute vec4 aVertexColor;
uniform float uPointSize;
uniform mat4 uModelViewMatrix;
uniform mat4 uProjectionMatrix;

varying lowp vec4 vColor;

void main(void) {
  gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
  vColor = aVertexColor;
  gl_PointSize = uPointSize;
}