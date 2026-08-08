from PIL import Image

# Create a valid 128x128 RGBA icon
img = Image.new("RGBA", (128, 128), (59, 130, 246, 255))
img.save("src-tauri/icons/icon.png")
