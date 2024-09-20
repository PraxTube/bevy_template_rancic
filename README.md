# Bevy Template Rancic

Highly opinionated template for bevy 2D top down games.

## Setup

Run `./setup.sh` and fill out the prompt. After that you should be ready to go.

## General Bevy Notes

### Animations

When doing anything with animations (trickfilm), you need to be careful with the scheduling. If you aren't, then you will likely get frame delays. This can result in visual glitches or potentially even outright bugs. So here is the guideline.

When updating animations (playing new animations) do it _before_ the `AnimationPlayer2DSystemSet`, which is the system in which the animations from trickfilm are updated. This way trickfilm will pick up your changes in the same frame and update them accordingly. If you were to run the call _after_, then you will get 1 frame delay which can look strange when flipping sprites for example.

Functions that deal with logic of animations (for example with `finished` or `just_finished`) may be run _after_ the `AnimationPlayer2DSystemSet`, though it may depend on your implementation (I don't know yet).
