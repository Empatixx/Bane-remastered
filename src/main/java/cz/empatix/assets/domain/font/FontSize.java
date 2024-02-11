package cz.empatix.assets.domain.font;

public enum FontSize {
    FONTSIZE_16(16),
    FONTSIZE_22(22),
    FONTSIZE_27(27),
    FONTSIZE_32(32);
    private final int size;
    FontSize(final int size) {
        this.size = size;
    }
    public int getSize() {
        return size;
    }
}