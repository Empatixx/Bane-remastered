mod draw;
mod font;
mod gpu;
mod light;
mod post;
mod quad;
mod texture;

pub(crate) use draw::{Draw, Layers};
pub(crate) use font::{Align, Anchor, FontSize, Fonts};
pub(crate) use gpu::Gpu;
pub(crate) use light::{Frame, LightSource, MAX_LIGHTS};
pub(crate) use post::PostParams;
pub(crate) use quad::{MapRoomLook, Quad};
#[cfg(test)]
pub(crate) use texture::Pixels;
pub(crate) use texture::TextureId;

#[cfg(test)]
mod shader_tests {
    fn validate(name: &str, source: &str) {
        let module = naga::front::wgsl::parse_str(source)
            .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(source)));
        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        );
        if let Err(error) = validator.validate(&module) {
            panic!("{name}: {error:?}");
        }
    }

    #[test]
    fn the_quad_shader_is_valid() {
        validate("quad.wgsl", include_str!("quad.wgsl"));
    }

    #[test]
    fn the_post_shader_is_valid() {
        validate("post.wgsl", include_str!("post.wgsl"));
    }
}
