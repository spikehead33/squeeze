<h1>Overview</h1>
Squeeze is a Compression command-line utility and library written in Rust<br><br>

<h1> HuffmanCompressor</h1>
HuffmanCompressor is a canonical huffman encoder/decoder.<br>
The file layout is as follow:<br><br>

|byte-no|short name|type|description|
|---|---|---|---|
|0|npad|u8|The number of padding bits at the end
|1|cb_size|u16|The MSB of the codebook size (byte 1 and byte 2 combine to form cb_size, a u16 variable)
|2|cb_size|u16|The LSB of the codebook size (byte 1 and byte 2 combine to form cb_size, a u16 variable)
|3..cb_size*2|codebook|(u8,u8)|pair of symbol and codeword mapping
|rest|compressed_content|[u8]|The compressed contents

The maximum possible size of the compressed file is 1+2+(256*2) = 515
<h1> Lz77Compressor</h1>