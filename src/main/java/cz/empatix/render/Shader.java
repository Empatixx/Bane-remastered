package cz.empatix.render;

import java.util.HashMap;

import static org.lwjgl.opengl.GL20.*;

public class Shader {
    private final int program;
    private final int fragmentShaderId;
    private final int vertexShaderId;
    private final HashMap<String,Integer> locations;

    public Shader(final String vs, final String fs) {
        program = glCreateProgram();
        locations = new HashMap<>();

        vertexShaderId = glCreateShader(GL_VERTEX_SHADER);
        glShaderSource(vertexShaderId, vs);
        glCompileShader(vertexShaderId);
        // if vertex shader is compiled wrong
        if(glGetShaderi(vertexShaderId,GL_COMPILE_STATUS) != 1){
            throw new RuntimeException(glGetShaderInfoLog(vertexShaderId));
        }

        fragmentShaderId = glCreateShader(GL_FRAGMENT_SHADER);
        glShaderSource(fragmentShaderId, fs);
        glCompileShader(fragmentShaderId);

        // if frag shader is compiled wrong
        if(glGetShaderi(fragmentShaderId,GL_COMPILE_STATUS) != 1){
            throw new RuntimeException(glGetShaderInfoLog(fragmentShaderId));
        }

        glAttachShader(program, vertexShaderId);
        glAttachShader(program, fragmentShaderId);

        glBindAttribLocation(program,0,"vertices");
        glBindAttribLocation(program,1,"textures");

        glLinkProgram(program);
        if(glGetProgrami(program,GL_LINK_STATUS) != 1){
            throw new RuntimeException(glGetProgramInfoLog(program));
        }
        glValidateProgram(program);
        if(glGetProgrami(program,GL_VALIDATE_STATUS) != 1){
            throw new RuntimeException(glGetProgramInfoLog(program));
        }
    }

    public int getShaderId() {
        return program;
    }
    public void destroy(){
        glDeleteShader(vertexShaderId);
        glDeleteShader(fragmentShaderId);
        glDeleteProgram(program);
    }
}
