package cz.empatix.assets.domain.font;

public enum FontSize {
    FONTSIZE_16(16),
    FONTSIZE_20(20),
    FONTSIZE_24(24);
    private final int size;
    FontSize(final int size) {
        this.size = size;
    }
    public int getSize() {
        return size;
    }
}