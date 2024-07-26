<div id="top"></div>

<br />
<div align="center">
  <h3 align="center">Coppybara - A copy utility written in rust</h3>
</div>

## Description

> ! This is a very early proof of concept

Coppybara is a copy utility written in rust, currently it only supports 'oc cp'.

## Build

You can build Coppybara using make and cargo, first verify that you have both installed.

```sh
cmake --version

#cmake version 3.30.0
#
#CMake suite maintained and supported by Kitware (kitware.com/cmake).

cargo --version

#cargo 1.80.0 (376290515 2024-07-16)
```

now run the steps below to compile and run the code

```sh
make # or make build

./target/release/coppybara
#error: the following required arguments were not provided:
#  <SRC>
#  <DEST>
#
#Usage: coppybara <SRC> <DEST>
#
#For more information, try '--help'.
```

## Usage

Run Coppybara just as you would use oc cp

```sh
coppybara remote-pod:/some/dir/archive.tar.gz ./archive.tar.gz
# ⠴ [00:00:04] archive.tar.gz [#>--------------------------------------] 21.93MiB/729.60MiB (5m)
```

## License

Coppybara is licensed under the [MIT License](https://opensource.org/license/mit).
