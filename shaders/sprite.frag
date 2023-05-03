#version 460 core

#include<flutter/runtime_effect.glsl>

precision mediump float;

layout(location=0)uniform sampler2D uTexture;
layout(location=1)uniform vec2 uTextureSize;

out vec4 fragColor;

void main(){
    vec2 currentPos=FlutterFragCoord().xy/uTextureSize;
    vec4 color=texture(uTexture,currentPos);
    // fragColor=color;
    // Turn rgb to grayscale
    float gray=dot(color.rgb,vec3(.299,.587,.114));
    fragColor=vec4(vec3(gray),color.a);
}