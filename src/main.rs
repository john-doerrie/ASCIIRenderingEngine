mod cl_drawer;
mod cl_map;
mod image_translator_2d;

fn main() {
    
    if let Some((w,h)) = term_size::dimensions() {
        println!("Width: {},\nHeight: {}", w, h);
    }else{
        println!("Width: 0, Height: 0 --> No command line is running");
    }
}