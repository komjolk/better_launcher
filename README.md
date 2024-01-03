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
            "color" : "blue"
        }
    ],
    "player": {
        "x": 100,
        "y": 900,
        "w": 100,
        "h": 100,
        "speed": 10,
        "jump_speed": 10,
        "gravity": 0.3,
        "color": "red"
    },
    "screen": {
        "w": 1000,
        "h": 1000,
        "color" : "black"
    }
}
```


Blocks stores all blocks in a list where x,y is the position of the top, and the right corner and h, w are height and width respectively. Command is a list of strings where the first string is the command to run and the rest are optional commands

Player is the moveable object where x,y is the start position of the top right corner. Speed is in acceleration where 1 is an acceleration of 1 pixel every 1/60th of a second same with gravity and jump_speed

Screen stores the data for the screen with w and h as width and height and color being the background color

## to do 
* Add support for pictures
* Don't spam command when touching multiple times
* Add support for select alike blocks
