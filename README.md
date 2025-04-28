# `physics`

Simple random related experiments.
I've also experimented with simulating and visualizing the wave-equation in both 2d and 3d in [@connorslade/wave-sim](https://github.com/connorslade/wave-sim) and [@connorslade/wave-sim-3d](https://github.com/connorslade/wave-sim-3d).

## `soft-body`

![Screenshot from 2025-04-20 at 18_11_39 491519035](https://github.com/user-attachments/assets/1b055004-5fbb-4dcb-9693-0cea4c864f88)

Simulates a bunch of point masses connected with springs making squishy soft bodies.
Their outlines are rendered with the catmull_rom spline.
Rendered with beam-engine, my custom 2d game engine / renderer from [Beam Time](https://github.com/connorslade/beam-time).

## `electrostatics`

![Screenshot from 2025-04-28 at 18_43_42 419916357](https://github.com/user-attachments/assets/45ba965a-feb3-41de-817c-e4609e897808)

Renders electric field and equipotential lines for point charges by using a shader to plot the result of two implicit equations.

The function $f(z,s)=s\ln{z}$ represents the complex potential of a source or sink (depending on the sign of $s$) with a strength determined by the magnitude of $s$. We can then add a few of these functions together with offset $z$ values to get multiple charged particles in the plot.

If we want to plot two point charges with opposite signs at ⟨-1,0⟩ and ⟨1,0⟩ we can write $f(-1,-1)+f(1,1)$.
By solving for the points that satisfy $\cos{n \Im{(f(-1,-1)+f(1,1))}}=0$ we see $n$ field lines, and similarly if we use the real component we get the equipotential lines.

## `gravity`

![Screenshot from 2025-04-28 at 18_44_20 277929331](https://github.com/user-attachments/assets/6bc93293-1513-4d1e-9189-965fb0463133)

Shows the affects of gravity on many small masses with negligible gravity of their own.
