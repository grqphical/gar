# gar - grqphical's Archive Format

This is a simple archive file format similar to TAR made with Rust. GAR does not apply any compression to the archives it creates

## Table of Contents
- [Schema](#schema)
- [Usage](#usage)

## Schema

A `gar` file consists of many file entries each with a 116 byte header where:

| Field Offset | Field Size | Description                                                                                                    |
|--------------|------------|----------------------------------------------------------------------------------------------------------------|
| 0            | 100        | The name of the file. If it is less than 100 bytes, null characters are added to it until it is 100 characters |
| 100          | 8          | The size of the file as a 32 bit hexadecimal number                                                            |
| 108          | 8          | The UNIX timestamp of when the file was last modified as a 32 bit hexadecimal number                           |

Bytes 116 to the size of the file are the contents of the file. This format repeats for each file in the archive

## Usage

To create a `gar` archive run:
```bash
$ gar create file1.txt file2.txt
```
You can add as many files as you want to the archive

To create an archive with a specific name use the `-o` flag:
```bash
$ gar create file1.txt file2.txt -o archive.gar
```

To list files currently inside of an archive run:
```bash
$ gar list archive.gar
```

To extract files from an archive run:
```bash
$ gar extract archive.gar
```

You also can specify an output directory with the `-o` flag
```bash
$ gar extract archive.gar -o foo
```

## License

gar is licensed under the MIT license