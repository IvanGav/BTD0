<!-- - Track movement:
  - True distance between every node is equal, meaning moving a bloon is as simple as adding a constant number to its position on the track. That number is based on the unit's speed and distance between two nodes.
  - For sharp corners to be possible, the hand placed nodes must live on a grid. All curves must snap to the closest possible approximation based on ideal number of nodes on the path.
  - Grid size may differ between maps. -->

<!-- - Actions every frame should be:
  - Move alive and despawn dead bloons and projectiles, spawn new bloons (probably fairly quick)
  - Move homing projectiles (probably fairly quick)
    - Need to access the newest positions of bloons this tick
  - Spawn projectiles (probably quick)
    - Don't move on this frame so have to happen after moving projectiles
    - If cmd only applies after this tick, can happen any time this tick up to this point
  - Check for bloon hits (potentially very slow)
    - Newly shot projectiles can collide so have to happen after spawning projectiles -->

- Assume a reasonable max amount of towers of 500
- Assume a reasonable max amount of bloons of 2,000
- Assume a reasonable max amount of projectiles of 50,000

- Actions every frame should be:
  1. Check for bloon hits (potentially very slow)
  2. Move bloons, non-homing projectiles/subtowers; spawn/despawn everything as needed
  3. Process homing projectiles/subtowers (heli, ace)
  - User input can be processed at any point (adding/removing monkeys, using abilities, retargeting, etc)

- Every projectile has a pierce limit
  - Every projectile tracks which bloons it has hit. After bounce, the list can be reset (or start of every tick for spike).
  - Every bloon has an ID - a number or a bevy ID can be used.
  - When a projectile hits a bloon, it records the ID and never hits that bloon again.

- For collision detection optimization:
  - Divide the entire field in quadrants. Each quadrant can be divided into quadrants too.
  - Only check bloons in quadrants that are theoretically hittable (projectile size + max bloon size away)

- Track movement:
  - Interpolate between the nodes and record true position and current quadrant

Things to keep in mind:
- Buffs/debuffs/status effects for both monkeys and bloons
- Insta-hit attacks (sauda, sniper, ice, explosions)
- Decamo/Delead/Deregrow (can probably be components that clear next tick)
- Non-circle attack hitboxes (sauda)
- Zombie bloons
- BLOWBACK AND OTHER FORMS OF KNOCKBACK
- Global effects