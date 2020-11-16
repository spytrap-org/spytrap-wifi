#!/usr/bin/env python3
import waveshare
from PIL import ImageDraw
from PIL import Image

e = waveshare.EPD()
e.init(e.FULL_UPDATE)
e.Clear(0xff)
e.init(e.PART_UPDATE)

WHITE = 0xff
BLACK = 0x00

width = 250
height = 122

class Log(object):
    stack = []

    def push(self, line):
        self.stack.append(line)
        self.stack = self.stack[-10:]
        self.update()

    def update(self):
        canvas = Image.new('1', (width, height), WHITE)
        drawer = ImageDraw.Draw(canvas)

        for (i, text) in enumerate(self.stack):
            drawer.text((0, i*12), text, font=None, fill=BLACK)
            print((i, text))

        img = canvas.rotate(180)
        buf = e.getbuffer(img)
        e.displayPartial(buf)


log = Log()
log.push('[#] booting...')

while True:
    x = input()
    log.push(x)
