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

Uh, there's a problem
When a bloon gets popped, none of its children can get popped by the same projectile. That's a problem, since I need to track each child's parents.
I also neglected to realize that layer skipping is a headache and a half to deal with.

New insights:
- Bloon modifiers can be represented by a bit mask. Projectile popping type can be represented with a bit mask as well.
  - Not a big issue rn, leave for later.
- Anything soaks though bloon layers. Select few things soak through blimp layers. Optimizing the soak for bloons might be a good idea.
  - How?
  - I can still use negative health as an overkill amount to transfer to children.
  - I should not spawn children right away. Instead, I need to calculate the true final result internally.
- For a faithful recreation, bloons only leave 1 child, after round 80. This can put more focus on optimizing for not having lots of small bloons too.
  - Yeah no, don't worry about it rn.
- Hitting child bloons after popping a parent is a big issue. I need to have an id for every bloon, since using Entity after it was despawned is invalid.
  - ~~Rather than going 0->MAX in numbers, I can use some sort of rng to give out ids to bloons.~~
  - ~~Wasted space, but oh well. I can't rely on despawned bloons' Entity id not being used again by bevy.~~
  - Scratch that, Entity has a generation counter. While it technically makes everything twice as long, whatever, really
  - Actually no, scratch that, I just realized that, for optimization reasons, I'm not summoning a new entity for the first child of a bloon; I'm transforming the parent into its first child. However that means that all other children of this bloon will know that a *still alive* bloon was their parent. Which it wasn't, but I can't change its Entity id. So making a u8 id would be much easier and just change it even for the same bloon.
- Money earning of regrows is messed up. I also want to have an option to remove the regrown bloons giving pops.
  - Yeah no just don't worry rn, it's actually messed up.

### Internals - how it actually works (or doesn't)

So empty =(