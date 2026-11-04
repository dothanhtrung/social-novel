#!/usr/bin/env python3
"""Local sticker and GIF pack for the comment box. Nothing is fetched from the network.

Needs Pillow and ImageMagick (`convert`). Run from the repo root:
    python3 res/assets/make_comment_pack.py
"""
import math
import os
import subprocess

from PIL import Image, ImageDraw

ROOT = os.path.dirname(os.path.abspath(__file__))
STICKER = os.path.join(ROOT, "stickers")
GIF = os.path.join(ROOT, "gifs")
S = 128

YELLOW = (255, 205, 80, 255)
YELLOW_EDGE = (230, 165, 35, 255)
BROWN = (90, 55, 30, 255)
WHITE = (255, 255, 255, 255)
BLUE = (24, 119, 242, 255)
RED = (230, 55, 70, 255)
PINK = (255, 105, 145, 255)
ORANGE = (255, 140, 40, 255)
GREEN = (70, 180, 90, 255)


def canvas():
    return Image.new("RGBA", (S, S), (0, 0, 0, 0))


def face(d, cx=64, cy=68, r=48, color=YELLOW):
    d.ellipse((cx - r, cy - r, cx + r, cy + r), fill=color, outline=YELLOW_EDGE, width=3)


def eyes(d, cx=64, cy=60, gap=16, r=5, color=BROWN):
    for x in (cx - gap, cx + gap):
        d.ellipse((x - r, cy - r, x + r, cy + r), fill=color)


def smile(d, box, color=BROWN, width=4):
    d.arc(box, 15, 165, fill=color, width=width)


def heart(d, cx, cy, s, color=RED):
    r = s * 0.28
    d.ellipse((cx - s * 0.52, cy - s * 0.42, cx - s * 0.02, cy + s * 0.08), fill=color)
    d.ellipse((cx + s * 0.02, cy - s * 0.42, cx + s * 0.52, cy + s * 0.08), fill=color)
    d.polygon(
        [(cx - s * 0.46, cy - s * 0.02), (cx + s * 0.46, cy - s * 0.02), (cx, cy + s * 0.5)],
        fill=color,
    )


def hand_up(color=YELLOW, edge=YELLOW_EDGE):
    """A thumbs-up, drawn upright then usable on any background."""
    hand = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    hd = ImageDraw.Draw(hand)
    hd.rounded_rectangle((50, 54, 86, 108), radius=14, fill=color, outline=edge, width=3)
    bar = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    bd = ImageDraw.Draw(bar)
    bd.rounded_rectangle((46, 22, 68, 74), radius=11, fill=color, outline=edge, width=3)
    bar = bar.rotate(32, resample=Image.BICUBIC, center=(58, 64))
    return Image.alpha_composite(hand, bar)


def sticker_like():
    im = canvas()
    d = ImageDraw.Draw(im)
    d.ellipse((10, 10, 118, 118), fill=BLUE)
    hand = hand_up(WHITE, (210, 225, 245, 255)).resize((96, 96), Image.LANCZOS)
    im.alpha_composite(hand, (16, 18))
    return im


def sticker_love():
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    heart(d, 46, 56, 28, RED)
    heart(d, 82, 56, 28, RED)
    smile(d, (40, 74, 88, 104))
    return im


def sticker_haha():
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    d.arc((36, 46, 58, 68), 200, 340, fill=BROWN, width=4)
    d.arc((70, 46, 92, 68), 200, 340, fill=BROWN, width=4)
    d.ellipse((44, 78, 84, 104), fill=BROWN)
    d.ellipse((50, 82, 78, 96), fill=(255, 120, 120, 255))
    d.polygon([(38, 52), (34, 78), (44, 58)], fill=(120, 190, 255, 255))
    d.polygon([(90, 52), (94, 78), (84, 58)], fill=(120, 190, 255, 255))
    return im


def sticker_wow():
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    eyes(d, r=8)
    d.ellipse((54, 82, 74, 104), fill=BROWN)
    d.ellipse((58, 86, 70, 98), fill=(255, 140, 140, 255))
    return im


def sticker_sad():
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    eyes(d, cy=58)
    d.arc((40, 90, 88, 112), 200, 340, fill=BROWN, width=4)
    d.polygon([(42, 64), (38, 88), (48, 70)], fill=(120, 190, 255, 255))
    return im


def sticker_angry():
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d, color=(255, 120, 90, 255))
    d.line((40, 46, 56, 54), fill=BROWN, width=4)
    d.line((88, 46, 72, 54), fill=BROWN, width=4)
    eyes(d, cy=64, r=4)
    d.arc((42, 86, 86, 108), 200, 340, fill=BROWN, width=4)
    return im


def sticker_care():
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    d.arc((36, 48, 58, 70), 200, 340, fill=BROWN, width=4)
    d.arc((70, 48, 92, 70), 200, 340, fill=BROWN, width=4)
    smile(d, (42, 76, 86, 100))
    heart(d, 92, 88, 22, PINK)
    return im


def sticker_cool():
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    d.rounded_rectangle((30, 50, 98, 70), radius=6, fill=(40, 40, 45, 255))
    d.rectangle((58, 54, 70, 66), fill=(180, 210, 255, 255))
    smile(d, (42, 78, 86, 100))
    return im


def sticker_kiss():
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    eyes(d)
    d.ellipse((52, 82, 76, 100), fill=PINK)
    heart(d, 100, 36, 24, RED)
    return im


def sticker_cry():
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    d.arc((36, 50, 56, 68), 20, 160, fill=BROWN, width=4)
    d.arc((72, 50, 92, 68), 20, 160, fill=BROWN, width=4)
    d.arc((42, 88, 86, 108), 200, 340, fill=BROWN, width=4)
    d.polygon([(40, 68), (34, 98), (48, 74)], fill=(120, 190, 255, 255))
    d.polygon([(88, 68), (94, 98), (80, 74)], fill=(120, 190, 255, 255))
    return im


def sticker_party():
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    eyes(d)
    smile(d, (40, 76, 88, 102))
    d.polygon([(40, 36), (64, 8), (88, 36)], fill=(90, 140, 255, 255))
    d.polygon([(52, 28), (64, 12), (76, 28)], fill=YELLOW)
    for x, y, c in ((22, 28, RED), (104, 24, BLUE), (18, 96, GREEN), (108, 100, PINK)):
        d.ellipse((x, y, x + 8, y + 8), fill=c)
    return im


def sticker_sleep():
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    d.arc((36, 52, 58, 70), 200, 340, fill=BROWN, width=4)
    d.arc((70, 52, 92, 70), 200, 340, fill=BROWN, width=4)
    d.arc((44, 82, 84, 98), 10, 170, fill=BROWN, width=3)
    d.text((92, 18), "z", fill=BLUE)
    d.text((104, 8), "Z", fill=BLUE)
    return im


def sticker_heart():
    im = canvas()
    d = ImageDraw.Draw(im)
    heart(d, 64, 62, 96, RED)
    return im


def flame(d, cx, top, color):
    d.ellipse((cx - 28, 58, cx + 28, 116), fill=color)
    d.pieslice((cx - 34, top, cx + 34, 100), 200, 340, fill=color)
    d.polygon([(cx - 26, 78), (cx, top + 4), (cx + 26, 78)], fill=color)


def sticker_fire():
    im = canvas()
    d = ImageDraw.Draw(im)
    flame(d, 64, 12, RED)
    flame(d, 64, 28, ORANGE)
    d.ellipse((48, 72, 80, 108), fill=YELLOW)
    d.polygon([(52, 86), (64, 46), (76, 86)], fill=YELLOW)
    return im


def sticker_star():
    im = canvas()
    d = ImageDraw.Draw(im)
    pts = []
    for i in range(10):
        ang = -math.pi / 2 + i * math.pi / 5
        r = 54 if i % 2 == 0 else 22
        pts.append((64 + r * math.cos(ang), 66 + r * math.sin(ang)))
    d.polygon(pts, fill=YELLOW, outline=YELLOW_EDGE)
    return im


def sticker_ok():
    im = canvas()
    d = ImageDraw.Draw(im)
    d.ellipse((14, 14, 114, 114), fill=GREEN)
    d.line((38, 66, 56, 86), fill=WHITE, width=10)
    d.line((56, 86, 92, 42), fill=WHITE, width=10)
    return im


def sticker_clap():
    im = canvas()
    d = ImageDraw.Draw(im)
    d.rounded_rectangle((12, 52, 58, 112), radius=16, fill=YELLOW, outline=YELLOW_EDGE, width=3)
    d.rounded_rectangle((70, 52, 116, 112), radius=16, fill=YELLOW, outline=YELLOW_EDGE, width=3)
    for x in (16, 28, 40):
        d.rounded_rectangle((x, 28, x + 14, 62), radius=7, fill=YELLOW, outline=YELLOW_EDGE, width=2)
    for x in (74, 86, 98):
        d.rounded_rectangle((x, 28, x + 14, 62), radius=7, fill=YELLOW, outline=YELLOW_EDGE, width=2)
    return im


def sticker_wave():
    hand = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    d = ImageDraw.Draw(hand)
    d.rounded_rectangle((46, 52, 86, 116), radius=16, fill=YELLOW, outline=YELLOW_EDGE, width=3)
    for x in (40, 54, 68, 82):
        d.rounded_rectangle((x, 24, x + 14, 64), radius=7, fill=YELLOW, outline=YELLOW_EDGE, width=2)
    d.rounded_rectangle((28, 64, 52, 92), radius=10, fill=YELLOW, outline=YELLOW_EDGE, width=2)
    return hand.rotate(-18, resample=Image.BICUBIC, center=(66, 86))


STICKERS = {
    "like": sticker_like,
    "love": sticker_love,
    "haha": sticker_haha,
    "wow": sticker_wow,
    "sad": sticker_sad,
    "angry": sticker_angry,
    "care": sticker_care,
    "cool": sticker_cool,
    "kiss": sticker_kiss,
    "cry": sticker_cry,
    "party": sticker_party,
    "sleep": sticker_sleep,
    "heart": sticker_heart,
    "fire": sticker_fire,
    "star": sticker_star,
    "ok": sticker_ok,
    "clap": sticker_clap,
    "wave": sticker_wave,
}


def frame_like(i):
    im = canvas()
    d = ImageDraw.Draw(im)
    scale = 0.92 + 0.1 * (0.5 + 0.5 * math.sin(i / 8 * math.tau))
    r = int(50 * scale)
    d.ellipse((64 - r, 64 - r, 64 + r, 64 + r), fill=BLUE)
    hand = hand_up(WHITE, (210, 225, 245, 255))
    side = int(78 * scale)
    hand = hand.resize((side, side), Image.LANCZOS)
    im.alpha_composite(hand, ((S - side) // 2, (S - side) // 2 + 2))
    return im


def frame_love(i):
    im = canvas()
    d = ImageDraw.Draw(im)
    s = 70 + 16 * math.sin(i / 8 * math.tau)
    heart(d, 64, 64, s, RED)
    return im


def frame_haha(i):
    im = canvas()
    d = ImageDraw.Draw(im)
    dx = int(6 * math.sin(i / 8 * math.tau))
    face(d, cx=64 + dx)
    d.arc((36 + dx, 46, 58 + dx, 68), 200, 340, fill=BROWN, width=4)
    d.arc((70 + dx, 46, 92 + dx, 68), 200, 340, fill=BROWN, width=4)
    d.ellipse((44 + dx, 78, 84 + dx, 104), fill=BROWN)
    drop = 50 + i * 6
    d.polygon([(38 + dx, drop), (34 + dx, drop + 22), (46 + dx, drop + 6)], fill=(120, 190, 255, 255))
    return im


def frame_wow(i):
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    r = 6 + int(3 * (0.5 + 0.5 * math.sin(i / 8 * math.tau)))
    eyes(d, r=r)
    mouth = 8 + r
    d.ellipse((64 - mouth, 86, 64 + mouth, 86 + mouth + 6), fill=BROWN)
    return im


def frame_sad(i):
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    d.arc((36, 50, 56, 68), 20, 160, fill=BROWN, width=4)
    d.arc((72, 50, 92, 68), 20, 160, fill=BROWN, width=4)
    d.arc((42, 90, 86, 110), 200, 340, fill=BROWN, width=4)
    y = 66 + (i % 8) * 5
    d.polygon([(40, y), (34, y + 18), (48, y + 4)], fill=(120, 190, 255, 255))
    return im


def frame_wave(i):
    hand = sticker_wave()
    angle = int(14 * math.sin(i / 8 * math.tau))
    return hand.rotate(angle, resample=Image.BICUBIC, center=(66, 90))


def frame_party(i):
    im = canvas()
    d = ImageDraw.Draw(im)
    face(d)
    eyes(d)
    smile(d, (40, 76, 88, 102))
    d.polygon([(40, 36), (64, 8), (88, 36)], fill=(90, 140, 255, 255))
    colors = (RED, BLUE, GREEN, PINK, YELLOW, ORANGE)
    for n in range(6):
        x = 12 + n * 18
        y = (i * 10 + n * 16) % 110
        d.ellipse((x, y, x + 8, y + 8), fill=colors[n])
    return im


def frame_fire(i):
    im = canvas()
    d = ImageDraw.Draw(im)
    bob = int(6 * math.sin(i / 8 * math.tau))
    flame(d, 64, 10 + bob, RED)
    flame(d, 64, 26 + bob, ORANGE)
    d.ellipse((48, 74, 80, 110), fill=YELLOW)
    d.polygon([(52, 88), (64, 44 + bob), (76, 88)], fill=YELLOW)
    return im


GIFS = {
    "like": frame_like,
    "love": frame_love,
    "haha": frame_haha,
    "wow": frame_wow,
    "sad": frame_sad,
    "wave": frame_wave,
    "party": frame_party,
    "fire": frame_fire,
}


def save_gif(name, draw_frame):
    tmp = os.path.join("/tmp", "comment-gif-" + name)
    os.makedirs(tmp, exist_ok=True)
    paths = []
    for i in range(8):
        path = os.path.join(tmp, f"{i:02d}.png")
        draw_frame(i).save(path)
        paths.append(path)
    out = os.path.join(GIF, name + ".gif")
    subprocess.check_call(
        ["convert", "-delay", "8", "-loop", "0", "-dispose", "Background", *paths, out]
    )


def main():
    os.makedirs(STICKER, exist_ok=True)
    os.makedirs(GIF, exist_ok=True)
    for name, draw in STICKERS.items():
        draw().save(os.path.join(STICKER, name + ".png"))
    for name, draw in GIFS.items():
        save_gif(name, draw)
    print(f"wrote {len(STICKERS)} stickers and {len(GIFS)} gifs")


if __name__ == "__main__":
    main()
