package cz.empatix.assets.providers;

import cz.empatix.assets.domain.RawShader;
import cz.empatix.assets.domain.ShaderType;
import cz.empatix.assets.loaders.ShadersLoader;
import cz.empatix.render.domain.Shader;
import cz.empatix.render.domain.ShaderProgram;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static org.lwjgl.opengl.GL20.GL_FRAGMENT_SHADER;
import static org.lwjgl.opengl.GL20.GL_VERTEX_SHADER;

public class ShaderProgramProvider implements AssetProvider {
    private static final Logger LOGGER = LoggerFactory.getLogger(ShaderProgramProvider.class);
    private final Map<String, ShaderProgram> shaders;
    private final ShadersLoader shadersLoader;

    public ShaderProgramProvider() {
        this.shaders = new HashMap<>();
        shadersLoader = new ShadersLoader();
    }

    public ShaderProgram get(final String name) {
        return shaders.get(name);
    }

    @Override
    public void create() {
        final long start = System.currentTimeMillis();
        int totalInited = 0;
        while(!shadersLoader.getAssets().isEmpty()){
            final List<RawShader> rawShaders = shadersLoader.getAssets().poll();
            final Shader fragmentShader = new Shader(shaderForFragment(rawShaders).content(), GL_FRAGMENT_SHADER);
            final Shader vertexShader = new Shader(shaderForVertex(rawShaders).content(), GL_VERTEX_SHADER);
            final ShaderProgram shaderProgram = new ShaderProgram(List.of(vertexShader, fragmentShader));
            shaders.put(rawShaders.get(0).getName(), shaderProgram);
            totalInited++;
        }
        final long end = System.currentTimeMillis();
        LOGGER.info("Inited " + totalInited + " shaders in " + (end - start) + "ms");

    }
    private RawShader shaderForFragment(final List<RawShader> rawShaders){
        return rawShaders.stream()
                .filter(r -> r.type().equals(ShaderType.FRAGMENT))
                .findFirst()
                .orElseThrow();
    }
    private RawShader shaderForVertex(final List<RawShader> rawShaders){
        return rawShaders.stream()
                .filter(r -> r.type().equals(ShaderType.VERTEX))
                .findFirst()
                .orElseThrow();
    }

    @Override
    public void destroy() {
        LOGGER.info("Destroying shaders");
        shaders.values().forEach(ShaderProgram::destroy);
    }
    @Override
    public void load() {
        shadersLoader.load();
    }

    @Override
    public void preload() {
        shadersLoader.preload();
    }
}

