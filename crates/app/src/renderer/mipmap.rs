use crate::prelude::*;

pub fn gen_mips(image: &DynamicImage) -> Result<Vec<Vec<u8>>> {
	let size: vec2u = image.dimensions().into();
	if !(size.iter().all(u32::is_power_of_two)) {
		bail!("gen_mips: image size is not a power of 2: {:?}", size);
	}

	let mut mips = vec![image.to_rgba8().to_vec()];

	let mut size = size;
	let mut image = image.clone();

	while size.iter().all(|dim| dim > 1) {
		size = size / 2;
		image = image.resize_exact(size.x(), size.y(), image::imageops::FilterType::Triangle);
		mips.push(image.to_rgba8().to_vec());
	}

	Ok(mips)
}
