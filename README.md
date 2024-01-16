# Fractals

![Julia](assets/julia.gif)

Small desktop app where you can explore fractals.

The color of each pixel is computed using the corresponding number in the Complex plane and optionally using a seed controlled by the user.

## Rendering
Frames are rendered on the CPU using double presicion floats to allow zooming up to 10-15 orders of magnitude. However, the performance may be lower when using complex fractal rules like the ones based on Newton's method. For fractals based on polynomial reccurence relatios (Julia/Mandelbrot), my frame rate was usually above 60.

To speed up rendering, each frame is split into chunks to be rendered by a thread pool. Additionally, complex number operations are vectorized using SIMD.

## Controls
* W/A/S/D - translate the view window in the Complex plane
* Arrow Keys - translate the seed in the Complex plane
* Q - toggle stat display
* R - reset picture
* Mouse Wheel - zoom
