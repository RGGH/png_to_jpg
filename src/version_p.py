from PIL import Image
import os
from multiprocessing import Pool
import time

def process_image(input_path, output_dir):
    # Open the input file
    with Image.open(input_path) as img:
        # Convert the image to RGB
        rgb_img = img.convert('RGB')
        
        # Resize the image to 224x224 pixels using BILINEAR resampling
        resized_img = rgb_img.resize((224, 224), Image.BILINEAR)
        
        # Create the output file path
        output_path = os.path.join(output_dir, os.path.basename(input_path))
        
        # Save the resized image as a JPEG to the output file
        resized_img.save(output_path, format='JPEG')
        
        print(f"Processed and saved: {output_path}")

def process_directory(input_dir, output_dir):
    os.makedirs(output_dir, exist_ok=True)
    
    files = [os.path.join(input_dir, f) for f in os.listdir(input_dir) if os.path.isfile(os.path.join(input_dir, f))]
    
    with Pool() as pool:
        pool.starmap(process_image, [(file, output_dir) for file in files])

if __name__ == "__main__":
    input_dir = './input_images'
    output_dir = './output_images'

    # Start the timer
    start_time = time.time()

    process_directory(input_dir, output_dir)

    # Stop the timer and print the elapsed time
    elapsed_time = time.time() - start_time
    print(f"Processing completed in {elapsed_time:.2f} seconds")

