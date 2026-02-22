# Pajama Neuro (WIP)

This is a WIP project that will allow for remote play of the game *Pajama Sam in "No need to hide when it's dark outside"* by the AI Neuro-Sama.

## Installation Guide

1. Go to where Pajama Sam is installed
2. Enter the ScummVM_Windows folder
3. Rename SDL2.dll to SDL2_orig.dll
4. Put the replacement SDL2.dll from either the artifacts or a release into this folder
5. Launch Pajama Sam as normal

## Todo List for programming:

- [X] ScummVM Loads Mod
- [X] Mod can see what room we're currently in
- [X] Mod can affect the game
- [ ] Mod knows what's in Sam's inventory
- [X] Mod knows what actors are in each room
- [X] List of actions are available for what Neuro can interact with
- [ ] Mod knows the state of the room
- [ ] Mod handles cutscenes

## Contribution Guide:

### Room Contribution
This is for if you want to help out by adding a room. A room is made up of a few pieces, the room description and the object descriptions.

The room description has stuff about that specific room, like the name, id, and list of objects.
Then each object has a name & id.

#### But how do I find the ID of a room?

1. Open Pajama Sam
    - Open the game folder in steam
    - Then go into ScummVM_Windows
    - Install the mod (I guess technically optional)
    - Run scummvm.exe
        - If you installed the mod correctly, a command prompt should appear
    - add a new game, and when the file select appears, select the parent folder
    - then start the game

2. With the game selected, press Ctrl + D to open the debug prompt
3. In the command prompt window, you can type "room <replace with your id>" to go to that room

#### How do I find what objects are in a room?

1. Open the debug prompt using Ctrl + D
2. Type in "objects"
3. This should give you a list of every object in the room

#### How do I know which object is which?

To do this, I made (an LLM made) a python script that takes this readout and converts it to a png with the hitboxes in it. Now using the bounds and the hitboxes, you should be able to figure out what everything is based on where it is in your screen. Personally I copy the image into an image editing program like Paint.NET and overlay a screenshot of the game over it to easier tell what everything is.

Here's how to use the script:

1. Have python installed
2. Have Pillow installed via pip
3. run ``python debug_hitboxes.py``
4. copy and then paste in the readout you got from the ``objects`` debug command. Including the "Objects in current room" text.
5. Once you have pasted it, press Ctrl+Z and then Enter.
6. the screenshot should now be at ``room_hitboxes.png``

#### How do I add a room now that I have all the info?

Basically just copy what's in ``src/rooms/bedroom.rs`` and then change it to match with your room. Then add your room to the ``ROOMS`` slice in ``/src/rooms/mod.rs``. If you need help with this, feel free to ask me or any other contributor.

### Code Contribution

I've never really managed an open source project before so I don't really know how many contributions I'll get but basically make sure you're nice, follow the existing conventions, and that your code actually compiles and does what it says it does before you make a PR.