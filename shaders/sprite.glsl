#version 460 core

#include<flutter/runtime_effect.glsl>

precision mediump float;

layout(location=0)out vec4 fragColor;
layout(location=0)uniform sampler2D uTexture;
layout(location=1)uniform vec2 uTextureSize;

void main(){
    vec2 currentPos=FlutterFragCoord().xy/uTextureSize;
    vec4 color=texture(uTexture,currentPos);
    fragColor=color;
}