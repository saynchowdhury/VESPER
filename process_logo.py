import os
from PIL import Image, ImageOps

def make_transparent(img_path):
    # Open image and convert to RGBA
    img = Image.open(img_path).convert("RGBA")
    data = img.getdata()
    
    new_data = []
    for item in data:
        # If the pixel is very dark (black or near black), make it transparent
        if item[0] < 20 and item[1] < 20 and item[2] < 20:
            new_data.append((255, 255, 255, 0))
        else:
            # Otherwise, make it white (assuming it's the white part of the logo)
            new_data.append((255, 255, 255, item[3]))
            
    img.putdata(new_data)
    
    # Crop to bounding box
    bbox = img.getbbox()
    if bbox:
        img = img.crop(bbox)
        
    # Add padding
    w, h = img.size
    pad = int(max(w, h) * 0.1)
    new_w = w + 2 * pad
    new_h = h + 2 * pad
    
    padded_img = Image.new("RGBA", (max(new_w, new_h), max(new_w, new_h)), (255, 255, 255, 0))
    offset = ((padded_img.size[0] - w) // 2, (padded_img.size[1] - h) // 2)
    padded_img.paste(img, offset)
    
    return padded_img

def save_icon_sizes(base_img, output_dir, is_mac_asset=False, mac_asset_name=None):
    if not os.path.exists(output_dir):
        return
        
    if is_mac_asset and mac_asset_name:
        # Save as a single large PNG for mac asset
        resized = base_img.resize((1024, 1024), Image.Resampling.LANCZOS)
        out_path = os.path.join(output_dir, mac_asset_name)
        resized.save(out_path, "PNG")
        print(f"Saved {out_path}")
        return

    sizes = [16, 32, 48, 64, 128, 256, 512]
    for size in sizes:
        resized = base_img.resize((size, size), Image.Resampling.LANCZOS)
        out_path = os.path.join(output_dir, f"{size}x{size}.png")
        if os.path.exists(out_path):
            resized.save(out_path, "PNG")
            print(f"Saved {out_path}")
            
    # Also save as icon.ico
    ico_path = os.path.join(output_dir, "icon.ico")
    if os.path.exists(ico_path):
        base_img.save(ico_path, format="ICO", sizes=[(s,s) for s in [16, 32, 48, 64, 128, 256]])
        print(f"Saved {ico_path}")

def main():
    source_img = r"e:\VESPER\warp-source\VESPER.png"
    if not os.path.exists(source_img):
        print(f"Error: {source_img} not found.")
        return
        
    transparent_img = make_transparent(source_img)
    
    base_dir = r"e:\VESPER\warp-source\app\channels"
    channels = ["dev", "preview", "stable", "local", "oss"]
    
    mac_asset_names = {
        "dev": "PLACE HERE.png",
        "preview": "Preview.png",
        "stable": "4-2.png"
    }
    
    for channel in channels:
        no_padding_dir = os.path.join(base_dir, channel, "icon", "no-padding")
        if os.path.exists(no_padding_dir):
            save_icon_sizes(transparent_img, no_padding_dir)
            
        app_icon_dir = os.path.join(base_dir, channel, "icon", "AppIcon.icon", "Assets")
        if os.path.exists(app_icon_dir) and channel in mac_asset_names:
            save_icon_sizes(transparent_img, app_icon_dir, is_mac_asset=True, mac_asset_name=mac_asset_names[channel])

    # Landing page logo
    landing_page_logo = r"e:\VESPER\landingpage\public\logo.png"
    if os.path.exists(landing_page_logo):
        resized = transparent_img.resize((512, 512), Image.Resampling.LANCZOS)
        resized.save(landing_page_logo, "PNG")
        print(f"Saved {landing_page_logo}")

    landing_page_favicon = r"e:\VESPER\landingpage\public\favicon.ico"
    if os.path.exists(landing_page_favicon):
        transparent_img.save(landing_page_favicon, format="ICO", sizes=[(32, 32)])
        print(f"Saved {landing_page_favicon}")

if __name__ == "__main__":
    main()
