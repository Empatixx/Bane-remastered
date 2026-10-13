const BORDER: u32 = 1;
const MIN_STEP: u32 = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Rect {
    fn fits(&self, width: u32, height: u32) -> bool {
        width + BORDER <= self.width && height + BORDER <= self.height
    }

    fn order(&self) -> (u32, u32) {
        (self.height, self.width)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Packed {
    pub(super) size: u32,
    pub(super) positions: Vec<(u32, u32)>,
}

fn try_pack(sizes: &[(u32, u32)], order: &[usize], target: u32) -> Option<Vec<(u32, u32)>> {
    let mut free = vec![Rect {
        x: 0,
        y: 0,
        width: target,
        height: target,
    }];
    let mut positions = vec![(0, 0); sizes.len()];
    for &index in order {
        let (width, height) = sizes[index];
        let found = free.iter().position(|rect| rect.fits(width, height))?;
        let rect = free.remove(found);
        positions[index] = (rect.x, rect.y);
        let remaining_width = rect.width.saturating_sub(width + BORDER);
        let remaining_height = rect.height.saturating_sub(height + BORDER);
        let mut add = |rect: Rect| {
            let at = free.partition_point(|other| other.order() < rect.order());
            free.insert(at, rect);
        };
        if remaining_height > 0 {
            add(Rect {
                x: rect.x,
                y: rect.y + height + BORDER,
                width: rect.width,
                height: remaining_height,
            });
        }
        if remaining_width > 0 {
            add(Rect {
                x: rect.x + width + BORDER,
                y: rect.y,
                width: remaining_width,
                height: height + BORDER,
            });
        }
    }
    Some(positions)
}

fn smallest_for_order(sizes: &[(u32, u32)], order: &[usize], max_size: u32) -> Option<Packed> {
    let mut best = try_pack(sizes, order, max_size).map(|positions| Packed {
        size: max_size,
        positions,
    })?;
    let mut target = max_size;
    let mut step = max_size / 2;
    while step >= MIN_STEP {
        let attempt = target.saturating_sub(step).max(1);
        if let Some(positions) = try_pack(sizes, order, attempt) {
            target = attempt;
            best = Packed {
                size: attempt,
                positions,
            };
        }
        step /= 2;
    }
    Some(best)
}

pub(super) fn pack(sizes: &[(u32, u32)], max_size: u32) -> Option<Packed> {
    if sizes.is_empty() {
        return None;
    }
    let by = |key: fn(&(u32, u32)) -> u32| {
        let mut order: Vec<usize> = (0..sizes.len()).collect();
        order.sort_by(|a, b| key(&sizes[*b]).cmp(&key(&sizes[*a])).then(a.cmp(b)));
        order
    };
    let orders = [
        by(|(width, height)| width * height),
        by(|(width, _)| *width),
        by(|(_, height)| *height),
    ];
    orders
        .iter()
        .filter_map(|order| smallest_for_order(sizes, order, max_size))
        .min_by_key(|packed| packed.size)
}

pub(super) fn assemble(packed: &Packed, images: &[(u32, u32, &[u8])]) -> Vec<u8> {
    let size = packed.size as usize;
    let mut atlas = vec![0u8; size * size * 4];
    for (index, (width, height, rgba)) in images.iter().enumerate() {
        let (x, y) = packed.positions[index];
        let (x, y, width, height) = (x as usize, y as usize, *width as usize, *height as usize);
        for row in 0..height {
            let from = row * width * 4;
            let to = ((y + row) * size + x) * 4;
            atlas[to..to + width * 4].copy_from_slice(&rgba[from..from + width * 4]);
            if x + width < size {
                let edge = to + (width - 1) * 4;
                let copy: [u8; 4] = [
                    atlas[edge],
                    atlas[edge + 1],
                    atlas[edge + 2],
                    atlas[edge + 3],
                ];
                atlas[to + width * 4..to + width * 4 + 4].copy_from_slice(&copy);
            }
        }
        if y + height < size {
            let last = ((y + height - 1) * size + x) * 4;
            let below = ((y + height) * size + x) * 4;
            let width_end = (width + usize::from(x + width < size)) * 4;
            let copy: Vec<u8> = atlas[last..last + width_end].to_vec();
            atlas[below..below + width_end].copy_from_slice(&copy);
        }
    }
    atlas
}

#[cfg(test)]
mod tests {
    use super::*;

    fn overlaps(a: (u32, u32, u32, u32), b: (u32, u32, u32, u32)) -> bool {
        a.0 < b.0 + b.2 && b.0 < a.0 + a.2 && a.1 < b.1 + b.3 && b.1 < a.1 + a.3
    }

    fn assert_valid(sizes: &[(u32, u32)], packed: &Packed) {
        for (index, (width, height)) in sizes.iter().enumerate() {
            let (x, y) = packed.positions[index];
            assert!(x + width <= packed.size && y + height <= packed.size);
            for (other, ((ow, oh), (ox, oy))) in
                sizes.iter().zip(&packed.positions).enumerate().take(index)
            {
                assert!(
                    !overlaps((x, y, *width, *height), (*ox, *oy, *ow, *oh)),
                    "{index} overlaps {other}"
                );
            }
        }
    }

    #[test]
    fn nothing_overlaps_and_everything_fits_inside_the_atlas() {
        let sizes: Vec<(u32, u32)> = (1..40)
            .map(|index| (16 + index * 7 % 90, 16 + index * 13 % 70))
            .collect();
        let packed = pack(&sizes, 4096).expect("fits");
        assert_valid(&sizes, &packed);
    }

    #[test]
    fn the_atlas_shrinks_towards_the_content() {
        let sizes = vec![(100, 100), (100, 100), (100, 100), (100, 100)];
        let packed = pack(&sizes, 4096).expect("fits");
        assert!(packed.size < 512, "{}", packed.size);
        assert_valid(&sizes, &packed);
    }

    #[test]
    fn a_single_image_needs_one_pixel_of_spare_room() {
        assert!(try_pack(&[(64, 64)], &[0], 64).is_none());
        assert!(try_pack(&[(64, 64)], &[0], 65).is_some());
    }

    #[test]
    fn content_larger_than_the_limit_cannot_be_packed() {
        assert!(pack(&[(600, 600), (600, 600)], 1024).is_none());
        assert!(pack(&[], 1024).is_none());
    }

    #[test]
    fn assembling_copies_the_pixels_and_extends_the_edges_into_the_border() {
        let packed = Packed {
            size: 8,
            positions: vec![(1, 1)],
        };
        let red = [255u8, 0, 0, 255];
        let blue = [0u8, 0, 255, 255];
        let mut image = Vec::new();
        for _ in 0..2 {
            image.extend_from_slice(&red);
            image.extend_from_slice(&blue);
        }
        let atlas = assemble(&packed, &[(2, 2, image.as_slice())]);
        let pixel = |x: usize, y: usize| &atlas[(y * 8 + x) * 4..(y * 8 + x) * 4 + 4];
        assert_eq!(pixel(1, 1), red);
        assert_eq!(pixel(2, 1), blue);
        assert_eq!(pixel(3, 1), blue);
        assert_eq!(pixel(2, 3), blue);
        assert_eq!(pixel(3, 3), blue);
        assert_eq!(pixel(0, 0), [0, 0, 0, 0]);
    }
}
