use image::{io::Reader as ImageReader, GenericImageView};

fn main() {
    let scale = 15.0 as f32;
    let mapping = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";

    let mapping_array: Vec<char> = mapping.chars().collect();
    let mapping_array_len = mapping_array.len();

    let test_image = ImageReader::open("cat.jpg")
        .expect("Could not find the file")
        .decode()
        .expect("Could not decode the file contents");
    let (width, height) = test_image.dimensions();

    let resize_width = (width as f32 / scale) as u32;
    let resize_height = (height as f32 / scale) as u32;

    let resized_image = test_image.thumbnail(resize_width, resize_height);

    let mut ascii_art = String::new();

    let from_range = Range {
        start: 0.0,
        end: 255.0,
    };
    let to_range = Range {
        start: 0.0,
        end: mapping_array_len as f32,
    };

    for (x, _y, pixel) in resized_image.pixels() {
        let avg = pixel.0[0] / 3 + pixel.0[1] / 3 + pixel.0[2] / 3;

        ascii_art
            .push(mapping_array[map_ranges(&from_range, &to_range, avg as f32).floor() as usize]);

        if x == resize_width - 1 {
            ascii_art.push('\n');
        }
    }

    println!("{}", ascii_art);
}

struct Range {
    start: f32,
    end: f32,
}

fn map_ranges(from_range: &Range, to_range: &Range, value: f32) -> f32 {
    (value - from_range.start) / (from_range.end - from_range.start)
        * (to_range.end - to_range.start)
        + to_range.start
}
