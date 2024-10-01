- [x] Terrain generation
	- Fractal Perlin noise
	- [This video](https://www.youtube.com/watch?v=gsJHzBTPG0Y)
	- Water bodies
	- [This paper](https://www.cs.umd.edu/class/spring2018/cmsc425/Lects/lect13-2d-perlin.pdf)
	- River generation
		- Basically one big spline/bezier curve
		1) Randomly generate points on the outside of the terrain
		2) Linearly interpolate to get the other control points
		3) Randomly shift the control points
		4) Maybe add falloff?
	- Lake generation
		- Maybe do some gradient descent trickery?
		- Maybe just mark points beneath a certain threshold?
		- Maybe just use another perlin noise map?
- [x] Camera controlling
	- Relearn geometric algebra
- [ ] Resource generation
	1) Generate a handful of random points
		- Should be fairly close together
	2) Compute minimum of their SDF's
	3) Bound the result
- [ ] District mapping
	- Drawing out residential, commercial, industrial
	- Mapping cursor to 3d scene
- [ ] Road generation
	- Avoid sharp elevation changes
	- Avoid lakes/water
	- Should be grid-like, but still natural
	- Wave function collapse?
- [ ] Building generation
	- Stick to districts