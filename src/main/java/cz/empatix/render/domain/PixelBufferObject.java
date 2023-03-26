package cz.empatix.render.domain;

import org.lwjgl.opengl.GL32;

public record PixelBufferObject(
        int pboId,
        long fence
) {
    public static PixelBufferObject init(){
        final int id = GL32.glGenBuffers();
        final long fence = GL32.glFenceSync(GL32.GL_SYNC_GPU_COMMANDS_COMPLETE, 0);
        return new PixelBufferObject(id, fence);
    }
    public boolean isFencePassed() {
        if (fence == 0) {
            return false;
        }
        int status = GL32.glClientWaitSync(fence, 0, 0);
        return status == GL32.GL_ALREADY_SIGNALED || status == GL32.GL_CONDITION_SATISFIED;
    }

    public void destroy() {
        GL32.glDeleteBuffers(pboId);
        GL32.glDeleteSync(fence);
    }
}
