# better_launcher
A better way to open your apps, by using a Mario-like game

## to run
* Clone gh repo and download packages either by running nix-shell or downloading them manually.
* cargo run ~/path/to/config.json

## how to config
The config file is in JSON format with three objects:
```


{
    "blocks": [
        {
            "x": 100,
            "y": 800,
            "w": 100,
            "h": 100,
            "command": [
                "ls", "-A"
            ],
            "color" : [0,0,255]
        }
    ],
    "player": {
        "x": 100,
        "y": 900,
        "w": 100,
        "h": 100,
        "speed": 5,
        "jump_speed": 10,
        "gravity": 0.3,
        "color": [255,0,0],
        "friction": 1,
        "image": "player.png"
    },
    "screen": {
        "w": 1000,
        "h": 1000,
        "color" :  [0,0,0]
    }
}
```


Blocks stores all blocks in a list where x,y is the position of the top, and the right corner and h, w are height and width respectively. Command is a list of strings where the first string is the command to run and the rest are optional commands

Player is the moveable object where x,y is the start position of the top right corner. Speed is in acceleration where 1 is an acceleration of 1 pixel/s every 1/60th of a second same with gravity and jump_speed. If color is wanted instead of image write a non existing image.

Screen stores the data for the screen with w and h as width and height and color being the background color in rgb

## to do 
* Add support for select like blocks
* Add support for images for blocks
* Make the camera follow the player