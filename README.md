# TROPHIES

This is my discord bot for getting achievements with my friends :D 

### The end goal of this bot is as follows:
- [X]  `/achievement [title] [xp (int)]` - scores you an achievement
- [ ]  `/leaderboard` - shows you who has the most XP
- [X]  XP bar - you can level up
- [X]  once you get sufficiently high, you can *prestige*, resetting your 
       XP to 0, and granting you a permanent title
- [ ]  Balance XP gain & prestige bonuses
- [ ]  Allow you to edit titles once obtained?
- [ ]  Allow you to give XP to others as a gift?

### Setup
It should install the dependencies itself with `cargo build` or `cargo run`.  
Check out the documentation with `cargo docs`.  

To set the bot up for yourself, create a file named `.env`, and inside the file, write this,
using your token from the Discord developer portal:

``` dotenv
DISCORD_TOKEN=...
```

Then, run `/register`, and click the _"Register in Guild"_ button, and the slash commands 
should become available :)
