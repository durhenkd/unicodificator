# Unicodificator
This is a simple program to transform an image file into an UTF-8 randition in your terminal.

## Example:
```
> ./unicodificator trump.jpg 80 tri 20

Width: 63 | Height: 80, raw length:5040
      ▒        █   ▒▓████▓████▓███▓▓▓████████▓█▓▓▓▓▓▓▒░░ ░  ░░░
                   ░▓████████▓█████▓█████████████▓▓▓▓▒░     ░░░
     ░              ▒▓▓▓██████████████████████▓███▓▓▓▒░     ░░░
     ▓▒▓▒          ▒▓▓▓████████████████████████▓▓▓▓▓▓▒░░    ░░░
   ░▒▓▓▓░         ▓███▓▓██▓█████▓█████████████▓▓▓█▓▓▓▒░     ░░░
     █▒▓▒        ░▒███▓▓▓▓▓▓▒▒▒▒▓▓▓▓▓▓████████████▓▓▓▒░     ░░░
                  ░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓███████▓▓▓▓░░░   ░░░░
                  ▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓█▓▓▓███████▓▓▓░░░░  ░░░░
        ▒▓░      ░▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▓▓█████▓▓▓░░░░  ░░░░
      ░███▒░    ░░░▒▒░░░▒▒▒▒▒▓▓▓▓▓▓▓▓▓▒▒▒▒▒▓▓▓█▓▓▓▓▓▓░░░░  ░░░░
 ░   ▒█████▒░  ░▒▒▒▒▒░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓███▓▓▓▓▓▒░░░   ░░░░
 ░░▒████████▒  ░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓██▓▓▓▓▓▓▒░░░   ░░░░
 ░▒██████████▒░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░▒▓▓▓▓▓▓▓▓▓▒░░░   ░░░░
  ░▒██████████░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░▒░░░░░▒▒▒░░▒▓▓▓▓▓▓▓▓▒░░    ░░░░
 ░ ░▒█████████▓▒░░░▒▒▒▒▒░░░░░░░▒▓▒░░░░░░▒░▒▓▓▓▓▓▓▓▓▓▒░     ░░░░
 ░  ░▓█████████▒░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▓▒▒▒▒▒▒▒▒░▒▓▓▓▓▓▓▓▓▓▒░     ░░░░
 ░   ░████████▓█▒▒▒▒▒▒▒▒▒▓▒▒░░▒▓▓▓▒░░▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▒░     ░░░░
 ░    ░███████▓▓▓▒▒▒▒▒▒░░░░░░░░░░░░░░ ░░░▒▓▓▓▓▓▓▓▓▓▓▒░     ░░░░
 ░     ▒█████▓▓██▓░░▒▒▒░░░ ░░░░░░░░░░░░░▒▓▓▓▓▓▓▓▓▓▓▓▒░     ░░░░
        ▒████▓████▓▒▒▒░░░░▒▒░▒▒▒▒▒▒░░░░░▒▓▓▓▓▓▓▓▓▓▓▓▒░     ░ ░░
 ░░      ▒███▓████▓▒█▒░░░▒▒▒▒▒▒░░░▒▒▒░░▒▓██▓▓▓▓▓▓▓▓▒▒░       ░ 
 ░       ░███▓██▓░░▓█▓▒░░░▒▒▒▒▒▒▒▒▒▒░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓░░         
 ▒░       ▒▓▒░     ░███▓▒░░░░░░░░░░░▒░░░▒▒▓▓▓▓▓▓▓▓▓▓░░       ░░
 ░░                ░▓████▓▒░░░  ░░▒▓█▒      ░░▒▒▓▓▓▓░░░     ░▒▒
                    ░██████▓▓░  ▒▓███▒            ░▒░       ░░░
                     ▒█████▓░    ▒███▒                   ░     
                     ░████▓░     ░▒██▓                         
                      ▒██▓▒▓▒  ░▒▓▒▒█▓░                        
                      ░▓███▓░ ░░▓████▓▒    ▒▓                 ░
                       ▒███▓░ ░░▒█████▓░                      ░
                       ░███▓░░░░░▒█████                        
                        ▒██▓░░░░░░▒████░                       
                        ░██▓░░░░░░░▒███▒                       
                         ░█▓░░░░░░░░▓██▒                       
                          ▓█░░░░░░░░░██▓                       
                           █░░░░░░░░░▓█▓                       
                           ▓░░░░░░░░░▒█▒                       
                            ░░░░░░░░░░█░                       
                             ░░░░░░░░░▒                        
                             ░░░░░░░░░                         
	
```

## How it works

Simply, it makes use of the `image` crate to open your file and manipulate it (in a way specified by the parameters), and prints out the output.

Usage: `cargo run -- {file} {size} {filter} [constrast_modifier]`

- **file** - the file you want to render
- **size** - the size (in col or rows) of the output, the program will keep the aspect ratio
- **filter** - the filtering method for the scale down
- **contrast_modifier** *OPTIONAL* - a modifier for the contrast of the image before it's rendered, must be a floating point number (default: 0.0)
