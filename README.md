# configuration_convert
Configuration file format conversion, support json, toml and yaml format.

# usage
Currently supports the mutual conversion of the three configuration file formats of json, yaml and toml.
```shell
configuration file converter 0.0.1
wei zhikai <neepoowzk@gmail.com>
Different types of configuration file conversion, support (json, toml, yaml)

USAGE:
    config_convert --input_file_type <src_type> --output_file_type <dst_type> [file]

ARGS:
    <input file>    

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --input_file_type <src_type>     input configuration file type(json, toml, yaml)
    -d, --output_file_type <dst_type>    output configuration file type(json, toml, yaml)
```

# example
```shell
# read from files/example.json convert to toml format
./config_convert -s json -d toml files/example.json

# read from stdin
cat files/example.toml | ./config_convert -s toml -d yaml
```