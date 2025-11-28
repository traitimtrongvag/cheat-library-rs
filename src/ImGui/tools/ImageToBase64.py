import os
import base64

def image_to_base64(image_path):
    with open(image_path, "rb") as image_file:
        base64_encoded = base64.b64encode(image_file.read()).decode("utf-8")
    return base64_encoded

def save_base64_to_file(base64_string, output_file):
    with open(output_file, "w") as txt_file:
        txt_file.write(base64_string)

def main():
    image_path = input("Enter the path to the image file: ")
    base64_string = image_to_base64(image_path)

    image_name = os.path.basename(image_path)
    output_file = os.path.splitext(image_name)[0] + ".txt"

    save_base64_to_file(base64_string, output_file)
    print(f"Base64 encoded image saved to {output_file}")

if __name__ == "__main__":
    main()
