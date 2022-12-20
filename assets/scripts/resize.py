import argparse
import math
import os
from PIL import Image

def main():
    # Parse command line arguments
    parser = argparse.ArgumentParser(description='Resize image by repeating texture')
    parser.add_argument('input_file', help='Input image file')
    parser.add_argument('new_width', type=int, help='New width')
    parser.add_argument('new_height', type=int, help='New height')
    args = parser.parse_args()

    # Load input image
    input_image = Image.open(args.input_file)

    # Calculate number of times to repeat texture horizontally and vertically
    repeat_x = math.ceil(args.new_width / input_image.width)
    repeat_y = math.ceil(args.new_height / input_image.height)
    print(f'Repeating texture {repeat_x} times horizontally and {repeat_y} times vertically')

    # Create output image by repeating the input image
    output_image = Image.new('RGB', (repeat_x * input_image.width, repeat_y * input_image.height))
    for y in range(repeat_y):
        for x in range(repeat_x):
            output_image.paste(input_image, (x * input_image.width, y * input_image.height))

    # Crop output image to specified dimensions
    left = (output_image.width - args.new_width) // 2
    top = (output_image.height - args.new_height) // 2
    right = left + args.new_width
    bottom = top + args.new_height
    output_image = output_image.crop((left, top, right, bottom))

    # Generate output file name
    base, ext = os.path.splitext(args.input_file)
    output_file = f'{base}_{args.new_width}_{args.new_height}{ext}'

    # Check if output file already exists
    if os.path.exists(output_file):
        # Prompt user to confirm replacing the file
        response = input(f'{output_file} already exists. Do you want to replace it? [Y/n] ') or 'Y'
        if response.lower() != 'y':
            print('Operation cancelled')
            return
    
    # Save output image
    output_image.save(output_file)
    print(f'Success: Created {output_file}')

if __name__ == '__main__':
    main()
