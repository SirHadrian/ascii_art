use image::{io::Reader as ImageReader, GenericImageView};

fn main() {

    let scale = 10.0 as f32;
    let mapping = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";

    let mapping_array: Vec<char> = mapping.chars().collect();
    let mapping_array_len = mapping_array.len();

    let test_image=ImageReader::open("cat.jpg").expect("Could not find the file").decode().expect("Could not decode the file contents");
    let (width, height)=test_image.dimensions();

    let resize_width = (width as f32 / scale)as u32;
    let resize_height=(height as f32 / scale) as u32;

    let resized_image=test_image.thumbnail(resize_width, resize_height);


    for (x, y, pixel) in resized_image.pixels(){
        let avg = pixel.0[0]/3 + pixel.0[1]/3 + pixel.0[3]/3;
    }
}
