use image::{DynamicImage, ImageFormat, ImageReader};
use image::buffer::ConvertBuffer;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn main() {
    // Define the input and output directories
    let input_dir = Path::new("./input_images");
    let output_dir = Path::new("./output_images");

    // Create the output directory if it doesn't exist
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Collect paths of all files in the input directory
    let paths: Vec<PathBuf> = fs::read_dir(input_dir)
        .expect("Failed to read input directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Use an Arc to share the output directory among threads
    let output_dir = Arc::new(output_dir.to_path_buf());

    // Start the timer
    let start_time = Instant::now();

    // Create a thread for each image file
    let handles: Vec<_> = paths.into_iter().map(|input_path| {
        let output_dir = Arc::clone(&output_dir);
        thread::spawn(move || {
            process_image(&input_path, &output_dir);
        })
    }).collect();

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // Stop the timer and print the elapsed time
    let elapsed_time = start_time.elapsed();
    println!("Processing completed in {:?}", elapsed_time);
}

// Function to process a single image
fn process_image(input_path: &Path, output_dir: &Path) {
    // Open the input file
    let img = ImageReader::open(input_path)
        .expect("Failed to open input file")
        .decode()
        .expect("Failed to decode input file");

    // Convert the image to RGB
    let rgb_img = convert_to_rgb(img);

    // Resize the image to 224x224 pixels
    let resized_img = resize_image(rgb_img, 224, 224);

    // Create the output file path
    let output_path = output_dir.join(input_path.file_name().expect("Failed to get file name"));

    // Save the resized image as a JPEG to the output file
    save_image_as_jpeg(&resized_img, &output_path);

    println!("Processed and saved: {:?}", output_path);
}

// Function to convert an image to RGB
fn convert_to_rgb(img: DynamicImage) -> DynamicImage {
    match img {
        DynamicImage::ImageRgba8(img) => DynamicImage::ImageRgb8(img.convert()),
        DynamicImage::ImageRgba16(img) => DynamicImage::ImageRgb16(img.convert()),
        _ => img.to_rgb8().into(),
    }
}

// Function to resize an image
fn resize_image(img: DynamicImage, width: u32, height: u32) -> DynamicImage {
    img.resize_exact(width, height, image::imageops::FilterType::Triangle)
}

// Function to save an image as JPEG
fn save_image_as_jpeg(img: &DynamicImage, output_path: &Path) {
    img.save_with_format(output_path, ImageFormat::Jpeg)
        .expect("Failed to save image to output file");
}

