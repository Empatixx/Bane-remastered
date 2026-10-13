package cz.empatix.render.domain;

public interface FramebufferAttachment {
    void attach();
    void destroy();
    int getFlushingBits();
    void update(Framebuffer framebuffer);
}
