# Paige & Jane's First Friday Game Jam

For this game jam we were mostly focused on figuring out a process and learning
to work with the tools we're using for the game.

- We picked a theme and a genre separately
- The genre paige selected with a letsmakeagame.net and had it automatically
  pick a crossover of two genres.
    - We ended up getting QWOP-like x Stealth
- For the theme I asked my friends on discord to shout out ideas, they came up
  with quite a few:
    - seafaring
    - metal oxides
    - fungi
    - cats
    - history
    - haiku
    - the unbearable ordeal of being perceived
    - the ecstasy of being loved
    - the ecstasy of gold, by Morricone
    - harmonicas
    - and train robberies
    - gold lamé
- Then, another friend of ours @lirnril, who came over to hang out and help a
  bit with the game jam suggested that we combine two of the topics.
- Thanks to our friends and the merciless god that is RNG we set forth with a
  goal of creating a QWOP-like stealth train robbery game at sea...

## Jane - The Development

- I immediately started setting up the skeleton of the project while paige
  started brainstorming game designs and drawing concept art
- I picked GGEZ as my game engine of choice because I've heard great things
  about its approachability for beginners and wanted to minimize how much time
  I spent fighting error messages from a more complex game engine that I needed
  to use.
- I spent a bit of time looking at the various examples in the ggez repo to get
  an idea of how games are usually structured using this engine
- After around 10 minutes Paige came over with a drawing of a side scrolling
  platformer level based on a train car, with portholes, and clearly driving on
  top of the ocean.
- The main character was an octopus, which made me immediately suggest we name
  the game QWOPtopus.
- Paige described having the octopus move around, be able to grab the walls and
  swing around, and jump from platform to platform to collect treasure.
- I wasn't confident that I could implement the necessary physics within the
  given time constraints so I suggested that we instead have the octopus always
  stick to the wall.
- At this point, my friend @munin chimed in with a suggestion that instead of
  whatever idea I had been coming up with (Which I honestly can't even
  remember) we have the legs extend and retract. I liked this idea quite a lot
  and immediately ran with it.
- At this point I dove into implementing a movement system that I thought was
  sufficiently obtuse to deserve the name QWOPtopus, and paige went to work
  creating characters and further designing the game mechanics, though the
  movement system I ended up creating was not in anywhere near what paige had
  intended or wanted.
- I decided to use 4 anchor points each representing one of the QWOPtopus's
  legs.
- My initial control scheme was to use QWER to control the left legs, and UIOP
  to control the right legs.
- I then had the inner keys (ER and IO) retract each leg, and the outer keys
  (QW and OP) extend the legs
- I paired them up so that the outer keys from each set (QR and UP)
  each applied to the top legs, and the inner keys (WE and IO) each applied to
  the lower legs.
- In order to move you'd press one or more of the keys which would extend or
  retract that leg in the direction that it was currently facing, adding or
  subtracing a normalized copy of the vector between the center of the
  octopus's body and the leg's anchor point.
- Once all legs stopped moving it would re-center the octopus to the average
  location of all 4 leg's anchor points.
- I quickly ran into an issue where once you started moving in a given
  direction all the legs would line up and you'd no longer be able to move
  anywhere other than forwards or backwards in the same direction.
- I fixed this by adding a reset legs button via the space bar.
- This is essentially the finished game I committed after ~2.5 hours working on
  it.
- I did not get a chance to integrate any of paige's assets, and only used
  circles and lines to represent the QWOPtopus.
