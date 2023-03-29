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
    -u, --decompress                   decompress mode
    -V, --version                      Print version information
```
<br>
<h1> HuffmanCompressor</h1>
HuffmanCompressor is a canonical huffman encoder/decoder.<br>
The file layout is as follow:<br><br>

|byte-th|short name|type|description|
|---|---|---|---|
|0|npad|u8|The number of padding bits at the end
|1|msb_cbs|u8|The most significant byte of the codebook size (combine with the lsb_cbs to form a u16 codebook size).
|2|lsb_cbs|u8|The least significant byte of the codebook size
|3-cb_size*2|codebook|HashMap[u8,u8]|pair of symbol and codeword mapping
|rest|compressed_content|Vec[u8]|The compressed contents

The maximum possible size of the compressed file header is 1+2+(256*2) = 515 bytes
<br>
<h1> Lz77Compressor</h1>