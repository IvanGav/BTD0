# Bloons TD Fan Game (BTD6 Clone)

To formalize this project:
- BTD6 has lots of strange features - randomness for example - that the game would be better without.
- BTD6 is terribly optimized. Any top leaderboard timed boss or late game run will be extremely laggy.

I think I can fix both issues while having a little fun.

### Ideas/Thoughts - what I think will work well

- Assume a reasonable max amount of towers of 500
- Assume a reasonable max amount of bloons of 2,000
- Assume a reasonable max amount of projectiles of 50,000

- Actions every frame should be:
  1. Check for bloon hits (potentially very slow)
  2. Move bloons, non-homing projectiles/subtowers; spawn/despawn everything as needed
  3. Process homing projectiles/subtowers (heli, ace)
  - User input can be processed at any point (adding/removing monkeys, using abilities, retargeting, etc)

- Every projectile has a pierce limit
  - Every bloon has an ID - a number or a bevy ID can be used.
  - When a projectile hits a bloon, it records the ID and never hits that bloon again.
  - After bounce, the list can be reset (or start of every tick for spike).

- For collision detection optimization:
  - Divide the entire field in quadrants. Each quadrant can be divided into quadrants too.
  - Only check bloons in quadrants that are theoretically hittable (projectile size + max bloon size away)
    - For now, I will check every bloon/projectile pair and check if the quadrant is projectile size + bloon size away. If no, no need to do a proper check.
  - Assume that most projectiles will not hit many/any bloons. We can use events to send hits and parallelize collision checks.

- Track movement:
  - Interpolate between the nodes and record true position and current quadrant.
  - When a bloon is blown back, choose the closest node and set it as the target node. Everything will be done automatically.
    - Snapping to the closest node is not ideal. There should be a way to set a waypoint.

Things to keep in mind:
- Buffs/debuffs/status effects for both monkeys and bloons
- Insta-hit attacks (sauda, sniper, ice, explosions)
- Decamo/Delead/Deregrow
- Non-circle attack hitboxes (sauda)
- Zombie bloons
- Knockback/blowback
- Global effects

### Internals - how it actually works (or doesn't)

So empty =(