# File Sharing

This tool allows the user to easily share a file between two devices on the same network from the comfort of their terminal.


## Use

Launch the script on the device that has the file and scan the QR code or enter the URL in the browser on the destination device.\
By doing so the file will be downloaded to the target device.

```sh
> ./file-sharing -f path/to/my/file.txt
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


## Additional Features

- [ ] use the https protocol
- [ ] sharing large file
- [ ] sharing multiple files
- [ ] close after one download