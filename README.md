Tic Tac Toe
-------------------
A tik tak toe game bot.

A board works as follows -
  1. A Bot(either X or O) is selected randomly to go first.
  2. Bots mark board until a final condition is met.
  3. The last state of the board is displayed, along with winner and Bot that went first.


#### 1. Installing Rust
------------
```
curl https://sh.rustup.rs -sSf | sh
```

#### 2. Running
----------
cd into the repository, and execute
```
cargo run
```

#### 3. Final Conditions
--------

Note the following  
```"."``` is symbolic of ```"X"/"O"/"."```   
```"@"``` **must** be ```"X"/"O"```

Row win states
```
@ @ @    . . .   . . .
. . .    @ @ @   . . .
. . .    . . .   @ @ @
```

Column win states
```
@ . .    . @ .   . . @
@ . .    . @ .   . . @
@ . .    . @ .   . . @
```

Diagonal win states
```
@ . .    . . @   
. @ .    . @ .    
. . @    @ . .   
```