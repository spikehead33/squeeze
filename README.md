<h1>Overview</h1>
Squeeze is a Compression command-line utility and library written in Rust<br><br>

<h1>Demo</h1>

```
$ squeeze --help

squeeze 0.1.0

USAGE:
    squeeze [OPTIONS] --input <INPUT> --compressors <COMPRESSORS>

OPTIONS:
    -c, --compressors <COMPRESSORS>    compression algorithms; separate by spaces if pipelining
                                       compressors
    -h, --help                         Print help information
    -i, --input <INPUT>                input file path
    -o, --output <OUTPUT>              input file path
    -u, --is-uncompress                uncompress mode
    -V, --version                      Print version information
```

<h1> HuffmanCompressor</h1>
HuffmanCompressor is a canonical huffman encoder/decoder.<br>
The file layout is as follow:<br><br>

|byte-no|short name|type|description|
|---|---|---|---|
|0|npad|u8|The number of padding bits at the end
|1-2|cb_size|u16|The codebook size (byte 1 and byte 2 combine to form cb_size, a u16 variable)
|3-cb_size*2|codebook|HashMap[u8,u8]|pair of symbol and codeword mapping
|rest|compressed_content|Vec[u8]|The compressed contents

The maximum possible size of the compressed file header is 1+2+(256*2) = 515 bytes
<h1> Lz77Compressor</h1>