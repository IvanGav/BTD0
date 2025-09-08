- Assume a reasonable max amount of towers of 500
- Assume a reasonable max amount of bloons of 10,000
- Assume a reasonable max amount of projectiles of 200,000

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
  - I solved that with a recursive function, but: it can be made iterative, for smaller bloons i can just make a lookup table that's efficient.
- For a faithful recreation, bloons only leave 1 child, after round 80. This can put more focus on optimizing for not having lots of small bloons too.
  - Yeah no, don't worry about it rn.
- Hitting child bloons after popping a parent is a big issue. I need to have an id for every bloon, since using Entity after it was despawned is invalid.
  - ~~Rather than going 0->MAX in numbers, I can use some sort of rng to give out ids to bloons.~~
  - ~~Wasted space, but oh well. I can't rely on despawned bloons' Entity id not being used again by bevy.~~
  - Scratch that, Entity has a generation counter. While it technically makes everything twice as long, whatever, really
  - Actually no, scratch that, I just realized that, for optimization reasons, I'm not summoning a new entity for the first child of a bloon; I'm transforming the parent into its first child. However that means that all other children of this bloon will know that a *still alive* bloon was their parent. Which it wasn't, but I can't change its Entity id. So making a u8 id would be much easier and just change it even for the same bloon.
  - Ok I stayed in bed for over an hour just thinking about this issue. Using HashMaps and quadrating projectile-bloon lookup was really bothering me. And I succeeded. I came up with a way to make it linear projectile-bloon lookup, while using only 1 64 bit integer for bloons instead of a whole heap allocated hashmap. Here it goes:
    - Every spawned bloon has a 32 bit identifier. It also has an 5 bit "layer" number (up to 32 layers). The last 27 bits are the tree. The actual bit numbers can be played with.
    - id will be unique for every bloon entering the screen. If a projectile has hit a bloon with id 1 and is now hitting a bloon with id 2, it knows it's a different bloon.
    - When a bloon is popped, its children inherit the id.
    - The tree is a binary trie. Every node is named after its path to that node. Since I'm using pure integers, I also need to record current layer.
    - So, with (layer, tree) pair, root will be: (0,0). If it's a rainbow and it pops into two bloons, those two will have the same id and pairs (1,0) and (1,1).
    - Children of those two will be: (2,00), (2,01), (2,10), (2,11)
    - Given two nodes, if they have the same "id", take the lowest "layer". Mask "tree" up to the lowest "layer". If masked trees are the same, one of them was parent of another.
    - Logarithmic and VERY LOW space complexity in addition to constant retrieval time (assuming the "tree" is constant sized).
    - The only downside is that now bloons have a limit on their children. Moreover, spawning more than 2 children will require introducing "dummy nodes"
    - Say (layer,tree), (0,0) is a BAD. It spawns 5 children on pop.
    - It's children will be: (2,00),(2,01) are ZOMGs and (2,10),(3,110),(3,111) are DDTs
    - That's not great, as (1,0),(1,1),(2,11) are now stuck not representing bloons but just differentiating different children of the same bloon - "dummy nodes"
    - Although it's still extremely space efficient compared to alternatives. And masking/integer comparison is incomparable with string comparison. If the size is ever a problem, increasing the bits for "tree" to 32, 64 or 128 should fix any and all possible issues. Nobody in their right mind will mod such a deep nested bloon.
    - Bit magic shall carry me to victory.
    - I was wrong once again, it used to be linear based on children a bloon had. Now it's going to be linear based on bloons a projectile hit. Which is arguably worse. The only upside is no heap allocated vector for bloons' children.
    - Ehh, there might be a way around it. Problem: I am given a node and a list of nodes. I need to test if the given node is a child (part of subtree) of any node in the list. How can I store the nodes for optimal access time complexity.
- Money earning of regrows is messed up. I also want to have an option to remove the regrown bloons giving pops.
  - Yeah no just don't worry rn, it's actually messed up.

***

My bloon-projectile hit detection is inefficient. I'm not using quadrant idea yet, to be clear. Things to consider:
- Checking projectile-bloon collisions is the single most expensive oprtation so far.
- Some projectiles have low base pierce and checking if it has already hit a bloon is cheap.
- Some projectiles have high pierce and may have lots of bloons they have hit. It might be cheaper to check for overlap first.
- It's still true that most projectiles won't be hitting any bloons most of the time.
- Quardarnts idea will only work well if I can iterate over bloons in given quadrant. Otherwise bounding box calculation is cheaper.
- Bounding box - take the bloon/projectile size and check if they were squares they would overlap. Cheap.
  - If they do overlap, check if they actually actually hit.
  - If they do, check if has already hit.
  - It's likely that bounding box check will yield a true result (IS IT TRUE?). If that's true, checking for past hits before hypot check might be better? (probably not)
  1. Yup, can confirm, the above worked wonders. I used to check has_hit then true hit. The new order is so much better oh god.
- Back to the idea of quadrants, maybe use arenas to reset the bloon quadrants every tick and when calculating movement, put it all back?
  - Can potentailly skip any and all computations for projectiles far from bloons.
  - Is it common for projectiles to be far from most bloons? I think yes.
  1. With how good it runs now (look above) I don't want to do this *yet*.

I'm so overwhelmed by trying to plan ahead so much that I end up not making any decisions at all. Anyway.

Towers.
- When effects change, projectile stats need to change. But not on change detection. Only when the effect is added/removed.
- Theoretically I can make all effects as separate components
  - That won't work since I can have multiple of the same effect - multiple damage up effects for example

# Getting Ready to Rewrite the Entire Codebase LET'S GOOOO

Notes:
- When permabrew alch doesn't have any valid targets (all towers in range have the buff) it cannot check all towers for the buff every tick
  - maybe give him and only him some internal list of buffed towers, so that he doesn't check them again
- Subtowers can be made as children of their parent towers with bevy hierarchy
  - make sure to `cmd.entity(parent).remove_children(&[child_entity])` along with `cmd.entity(child).despawn()`
- Since `velocity` is now stored in `move along the road` movement type, when spawning a bloon that has to be put in there instead of stored in the bloon
- For the same reason I will need a function that will apply the status effects to bloons' movement component
- Immunity vs ignoring
  - For stuff like camo and poseidon+purple, the tower will not target the bloon; the stray shots will go over the bloon
  - That means, if can't see bloon, projectiles fly over the bloon
  - For dart+lead and such, the monkey will see and attack the bloon, but the bloon is immune to the projectile damage and will consume the projectile
- Disambiguation:
  - Bloon effects = (usually temporary) status effects (slow, weakness, on fire, etc)
  - Bloon modifiers = immunity to certain damage types (a projectile can pop a bloon iff `cannot_pop_modifiers` & `modifiers` == 0)
  - Bloon properties = special components that alter a bloon's behavior (Fortified, Regrow)
- When regrows regrow, they can essentially create unlimited amount of children of the same family. That is a bit of an issue.
  - I can't really solve this issue unless I move the remembering to bloons (no thanks) or do some jank solution where the family and tree will reset whenever it reaches the end of the tree. Jank, but better than nothing.

Movement:
- move along the track (bloons, unpopped army)
  - track_pos: f32, target_node: usize, waypoint: Vec2, velocity: f32
- simple move - move with a given velocity (any regular projectile)
  - velocity: Vec2, bounce: i32, collide_height: Option<f32> (if None, don't collide with environment; if Some, collide with environment up to given height level)
- move to waypoint and stay still (spike)
  - waypoint: Vec2 (remove this component when arrived at the destination)

Acceleration (only compatible with simple move; does nothing on different entities):
- steering - steering to a specific point
  - waypoint: Vec2, steer_str: f32, target_mode: TargetMode (specific entity, closest entity, etc)
- homing - change own velocity to move towards the target, up to a max velocity
  - waypoint: Vec2, home_str: f32, max_velocity: f32, target_mode: TargetMode (specific entity, closest entity, etc)
- seek after hit - after hitting a bloon, redirect self to face the next closest bloon (quincy i think)
  - hit_bit: bool

Hitboxes:
- simple hitbox (bloons, projectiles)
  - radius: f32
- composite hitbox (blimps)
  - radii: Vec<f32>, offsets: Vec<Vec2>

Projectile and related:
- lifetime - every tick decrement; despawn when reaches 0
  - lifetime: i32
- lifetime in rounds - every round decrement; despawn when reaches 0 (for permaspike and the one before it)
  - lifetime_rounds: i32
- damage dealer - every tick check for collisions with bloons; send events when damaging bloons
  - damage: i32, cannot_pop_modifiers: BloonModifier, cannot_target_modifiers: BloonModifier, hit_bloons: Vec<(family,layer,tree)>, pierce: u32

Bloon and related:
- bloon
  - (family,layer,tree), hp: i32 (negative is overkill amount), modifiers: BloonModifier, effects: Vec<BloonEffect>, tier: BloonTier (primarily for determining children)

Tower and related:
- attack - enum with many attacks
  - to spawn a projectile, has a function that takes cmd, transform, tower effects and something else and spawns the projectile
  - so, there will be a lot of functions such as `attack_dart_100`
- tower - subtowers are towers with Parent component
  - attacks: Vec<Attack>, targeting_modes: Vec<TargetingMode>, cur_targeting_mode: usize, effects: Vec<TowerEffect>

***

All good and dandy, but, towers are still just as much of a pain, actually.
- Attack types:
  - Single (dart, heli) - angle targeting, entity provided for homing
  - Instahit (sniper) - entity targeting
  - ArcSpread (sun avatar) - angle targeting, spread stat
  - Circle (ace)
  - Teleport (mortar)
- Projectile Stats:
  - Damage - can be buffed
  - Pierce - can be buffed
  - Velocity - can be affected by artifacts
  - Can hit/target types - can be buffed - cannot modify the stat directly, since cannot unmodify it later
- Attack Stats:
  - Range - can be buffed
  - Cooldown - can be buffed
- Additional Stats:
  - Current cooldown
  - Spread (multishot) - can be buffed

- Buff types:
  - Global (call to arms)
  - Area (village)
  - Single entity (alch, overclock)
- Buffs from same type of source can:
  - Stack (mermonkey, ninja)
  - Override (overclock)
  - ~~Ignore (alch)~~ - well, that's pretty much override; alch tower doesn't throw any more pots, but they would override
  - All stacking buffs are not direct (aka single entity), but global/area
  - Overriding buffs can be either global/are or direct (aka single entity)
- Buff duration:
  - Ticks (overclock)
  - Rounds (geraldo)
  - Permanent (alchbuff, village) (IMPORTANT: until removed)
  - When can buffs be taken away:
    - Duration ended
    - Buffing entity despawns (sold/sacrificed/etc)
  - Area buffs can be given when:
    - Tower is placed into an area buff range
    - Tower with area buff is placed near other towers

Having lots of enum types for every single feasable attack type is bothersome. 3 targeting types
- Angle (dart)
- Entity (sniper, any homing attacks)
- Waypoint (spike)
- Tower (Entity) (alch)
- RangeBuff (village)

Can be provided as:
- Angle + Entity (any bloon targeting options - FirstBloon, StrongBloon, LastBloon, CloseBloon, InRange)
- Waypoint (any road targeting options - CloseRoad, FarRoad, SmartRoad)
- None (Always targeting option, such as ace)
- Entity (any monkey targeting options - CloseTower)

Different **attacks** may want to track own **state**
- MAD alterating shots
- Desperado staggered shots

Different monkeys may want to have special conditions on their attacks
- Glue doesn't target glued bloons
- Alch doesn't target buffed monkeys
- Base ice can't target blimps


Bloon detection stats:
- Range
- Target types (lead,white,black,purple,etc)
- Non-glued (lower tiers of the same path are ignored)
- Non-alch-buffed
- Bloon/Blimp/Boss

Those Bloon Detection Stats are separate from the projectile. They are inherintly attack's responsibility.

What bloon will be shot is determined by Bloon Detection Stats. **They are aware of tower type/crosspath too**.
What projectile will be spawned is determined by a function that is unique to each tower path. **They are implicitly aware of tower type/crosspath**.

Functions that spawn projectiles will take:
- cmd: Commands
- angle: f32, e: Entity OR waypoint: Vec2
- effects: Vec<TowerEffect> OR some kind of struct reference that has pre-calculated buff stats
- state: &mut AttackState (some kind of enum that stores a unique state required for this attack)

Different towers want to have different behaviors:
- Clickable, upgradable tower
- Clickable, non-upgradable tower (cave monkey)
- Clickable, non-upgradable subtower (shooty turret)
- Non-clickable subtower (heli, ace, phoenix, buccaneer planes)

Also, detection range and attack range are different - advanced intel sub