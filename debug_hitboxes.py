import sys
import random
from PIL import Image, ImageDraw, ImageFont

def parse_objects(text):
    objects = []
    
    # 1. Anchor to the specific debug output
    marker = "Objects in current room"
    if marker in text:
        # Slice the text to only include everything after our marker
        text = text.split(marker)[1]
    else:
        print(f"Warning: Could not find '{marker}' in the pasted text.")
        print("Attempting to parse anyway...")

    # 2. Parse the table
    lines = text.strip().split('\n')
    for line in lines:
        # Stop parsing if we hit an empty line or a new debug command
        if line.strip() == "" and len(objects) > 0:
            break 
            
        if not line.startswith('|') or 'num' in line or '---' in line:
            continue
        
        parts = [p.strip() for p in line.split('|')]
        
        if len(parts) > 6:
            try:
                obj_id = int(parts[1])
                x = int(parts[3])
                y = int(parts[4])
                width = int(parts[5])
                height = int(parts[6])
                
                # Only keep objects that actually have a physical presence
                if width > 0 and height > 0:
                    objects.append({
                        "id": obj_id,
                        "x": x,
                        "y": y,
                        "w": width,
                        "h": height
                    })
            except ValueError:
                # If we hit a row that doesn't parse into integers, skip it
                continue
                
    return objects

def render_hitboxes(objects, output_file="room_hitboxes.png"):
    if not objects:
        print("No valid hitboxes found to render. Exiting.")
        return

    # Pajama Sam native resolution
    IMG_WIDTH = 640
    IMG_HEIGHT = 480
    
    img = Image.new('RGB', (IMG_WIDTH, IMG_HEIGHT), color=(30, 30, 30))
    draw = ImageDraw.Draw(img)
    font = ImageFont.load_default()

    for obj in objects:
        x0 = obj['x']
        y0 = obj['y']
        x1 = x0 + obj['w']
        y1 = y0 + obj['h']
        
        # Generate a random, bright color for each box
        color = (random.randint(100, 255), random.randint(100, 255), random.randint(100, 255))
        
        draw.rectangle([x0, y0, x1, y1], outline=color, width=2)
        
        text_bg_bounds = draw.textbbox((x0 + 4, y0 + 4), f"ID:{obj['id']}", font=font)
        draw.rectangle(text_bg_bounds, fill=(0, 0, 0, 180))
        draw.text((x0 + 4, y0 + 4), f"ID:{obj['id']}", fill=color, font=font)

    img.save(output_file)
    img.show()
    print(f"Rendered {len(objects)} hitboxes to {output_file}")

if __name__ == "__main__":
    print("Paste your ScummVM debug output below.")
    print("When you are done pasting, press Ctrl+Z and then hit Enter to render:")
    
    # Reads multi-line input from the terminal until EOF (Ctrl+Z on Windows)
    raw_input = sys.stdin.read()
    
    objs = parse_objects(raw_input)
    render_hitboxes(objs)