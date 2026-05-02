import struct, zlib

def make_png(w, h):
    def chunk(t, d):
        c = t + d
        return struct.pack('>I', len(d)) + c + struct.pack('>I', zlib.crc32(c) & 0xffffffff)
    raw = b''
    for y in range(h):
        raw += b'\x00'
        for x in range(w):
            raw += b'\x66\x7e\xea\xff'
    return b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', w, h, 8, 6, 0, 0, 0)) + chunk(b'IDAT', zlib.compress(raw)) + chunk(b'IEND', b'')

for n, s in [('32x32.png',32),('128x128.png',128),('128x128@2x.png',256),('icon.png',512)]:
    open(n,'wb').write(make_png(s,s))
    print(f'{n} OK')
