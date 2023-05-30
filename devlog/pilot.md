---
title: this is a short introduction
author: Kaique dos Santos
date: March 4, 2023
---

#### you don't need a story.
Rhythm games are so great in fact, that you don't even need a story. They do be different, don't they?  
It feels instinctive. Flickering lights and colors, the high-pitched fast paced sound keeps you alert, and present in the moment.
It has got to have something to do with some dopamine neurotransmitter thing.
I really wouldn't be able to tell you.

#### how come
I've never been very good at this game genre, I just like playing.
¯\\\_(ツ)\_/¯\
But like a scientist trying to maximize the pulp size of a fruit, I couldn't stop thinking about how come no one mix piano and rhythm games before.
It's a natural fit. You can't play piano without rhythm...\
Would it be possible to make a rhythm game that teaches you how to play the piano while having fun?
I can say right away it wouldn't make sense to have Osu teach me music theory in their tutorials.
But what if you could first play the piano game for the fun of its rhythm and tempo, and then later on, you could learn music theory.\
Can you make the learning piano experience more gamified, whilst not affecting the gameplay?

#### let's talk content
If you're creating a piano learning app, you'd need to fund teachers and record snippets of music theory and how to perform certain techniques.
That's why this project is a rhythm game, not a piano learning app.


I prefer going the other way 'round this problem, I want to rely on the community to create content and to give feedback on other's people play through.
That's why this game is going to be open source, I'll be taking suggestions and feedback from the community.
Making it easier for contributions, bug reports, content creation, and much more.

#### what's the plan
Play any of your .mid files.\
Use practice mode to loop specific difficult parts you're having trouble with.
Slow down the tempo, so you can learn at your own pace.\
When you're ready to tackle the song in full, you can play it in ranking mode, having your score go into the leaderboards.\
A replay system, so you can watch other people's play through and learn from them.

#### for the tech
I'm picking the rust language because I wanted to learn it. Although I'm not sure whether this is the best choice.\
ggez is the game framework of choice, I've seen something about it' being portable to the web and mobile, but I haven't put any effort into that.\
I'm using the midly crate to parse .mid files, midir to play midi files.\
And the voices inside my head to tell me what to do.

#### what's the progress
Let's cut to the chase, I've been working on this for a few months now, and I'm starting to wrap on the core gameplay.\
Here's what I got:

- PC keyboard input (mostly for debugging);
- a buggy midi file parsing;
- for midi playback;
	* input playback;
		+ for the notes you play;
	* song playback;
		+ for the notes you're supposed to play;
- note hit detection;
- a dumb note hit scoring system.

No UI or menu system yet, I'm focusing on the core gameplay first.
Up next is Piano input and improve the scoring system.

#### wrap up
You defo need to see what's coming up. Stay tuned as we plan for releasing.
Follow us on itch, as well as the youtube channel and github repo.\
Share your thoughts, suggestions, and especially feedback.
Don't forget to keep on piano-ing!
