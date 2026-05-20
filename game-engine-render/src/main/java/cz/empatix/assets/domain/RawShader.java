package cz.empatix.assets.domain;

import java.nio.file.Path;

public record RawShader (
        Path path,
        String content
){
    public ShaderType type(){
        if(path.toString().replace(".glsl", "").endsWith("vs")){
            return ShaderType.VERTEX;
        }else if(path.toString().replace(".glsl", "").endsWith("fs")){
            return ShaderType.FRAGMENT;
        }else{
            throw new IllegalArgumentException("Unknown shader type for path " + path);
        }
    }
    private final static String SUFFIX_PATTERN ="(-vs)*(-fs)*(\\.glsl)";
    public String getName(){
        return path.getFileName().toString().replaceAll(SUFFIX_PATTERN, "");
    }
}
