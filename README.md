# File Sharing

This tool allows the user to easily share a file between two devices on the same network from the comfort of their terminal.


## Use

Launch the script on the device that has the file, then scan the generated QR code or enter the URL in the browser on the destination device.\
By doing so the file will be downloaded to the target device.

```sh
> file-sharing -f path/to/my/file.txt
██████████████    ██  ██      ██████  ██    ██████████████
██          ██        ████          ██      ██          ██
██  ██████  ██  ██  ████    ██  ██  ████    ██  ██████  ██
██  ██████  ██  ██                ████████  ██  ██████  ██
██  ██████  ██  ██████    ██████      ██    ██  ██████  ██
██          ██  ██    ██████████    ██  ██  ██          ██
██████████████  ██  ██  ██  ██  ██  ██  ██  ██████████████
                ████  ██████  ████  ██                    
██  ██████████      ████    ████  ████  ██  ██████████    
      ██████          ██        ████  ██    ██  ██      ██
  ██████    ██  ██  ████      ████  ██        ████        
      ████    ██  ██████    ████    ████  ████  ██    ██  
  ████    ████  ████    ██    ██  ████  ████  ██  ████    
  ████  ████  ██      ██  ████  ██    ██    ██  ██  ██  ██
  ██  ████████  ██████    ████████  ██    ████      ██    
    ██████    ██    ██  ████    ██  ████████  ██      ██  
██  ██  ██████    ██        ██    ██████  ██        ██    
████          ██        ██      ████████  ████  ██████  ██
██  ████    ████  ██  ██      ██  ██        ██    ████    
██  ████  ██  ██  ██████    ██████        ████  ██    ██  
██  ██      ██  ██  ██████        ██  ████████████  ██████
                ██    ████████  ██  ██████      ██████████
██████████████        ██████████      ████  ██  ██████    
██          ██  ████████████    ██  ██  ██      ██      ██
██  ██████  ██  ██████  ██  ██    ██    ████████████████  
██  ██████  ██  ████████  ██    ████████████  ██      ████
██  ██████  ██  ████████    ████████  ██    ██  ████  ██  
██          ██    ████  ████    ██    ██████  ████    ██  
██████████████  ██      ██  ████████              ████    

Connect to: http://192.168.178.21:8080/download
```

## Features

- Share large files, tested up to 10GB
- Use your local network for file sharing, no internet access required

## Building & Installing

You will need to install [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to build this project.

While inside the repository execute the following command:

```sh
> cargo build --release
> cp target/release/file-sharing .
```

or if you want to install it to your system:

```sh
> cargo install --path .
```

Do not forget to add your cargo binary directory to your path variable.

## Future Features

- [ ] use the https protocol
- [x] sharing large file
- [ ] sharing multiple files
- [ ] close after one download
