package cz.empatix.assets.domain.font;

import java.nio.ByteBuffer;

public record RawGlyph (
        int width,
        int height,
        int bearingX,
        int advance,
        int ascent,
        int descent,
        int lineGap,
        byte[] bitmap,
        int bitmapWidth,
        int bitmapHeight
){
    public static Builder builder(){
        return new Builder();
    }
    public static class Builder{
        private int width;
        private int height;
        private int bearingX;
        private int advance;
        private int ascent;
        private int descent;
        private int lineGap;
        private int bitmapWidth;
        private int bitmapHeight;
        private byte[] bitmap;

        public Builder width(final int width){
            this.width = width;
            return this;
        }
        public Builder height(final int height){
            this.height = height;
            return this;
        }
        public Builder bearingX(final int bearingX){
            this.bearingX = bearingX;
            return this;
        }
        public Builder advance(final int advance){
            this.advance = advance;
            return this;
        }
        public Builder ascent(final int ascent){
            this.ascent = ascent;
            return this;
        }
        public Builder descent(final int descent){
            this.descent = descent;
            return this;
        }
        public Builder lineGap(final int lineGap){
            this.lineGap = lineGap;
            return this;
        }
        public Builder bitmap(final ByteBuffer bitmap){
            if(bitmap != null){
                this.bitmap = new byte[bitmap.remaining()];
                bitmap.get(this.bitmap);
            }
            return this;
        }
        public Builder bitmapWidth(final int bitmapWidth){
            this.bitmapWidth = bitmapWidth;
            return this;
        }
        public Builder bitmapHeight(final int bitmapHeight){
            this.bitmapHeight = bitmapHeight;
            return this;
        }
        public RawGlyph build(){
            return new RawGlyph(width, height, bearingX, advance, ascent, descent, lineGap, bitmap, bitmapWidth, bitmapHeight);
        }
    }
}
