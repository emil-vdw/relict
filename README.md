# relict
A Rust genetic algorithm framework.

## Example
Consider a simple `Point` struct:
https://github.com/vdwemil95/relict/blob/b920a008a02a83a89ba6f3d2dd687ce1d46c7838/examples/point_evolution.rs#L15-L19

With the following fitness function:
https://github.com/vdwemil95/relict/blob/b920a008a02a83a89ba6f3d2dd687ce1d46c7838/examples/point_evolution.rs#L31-L41
![fitness_function](https://github.com/vdwemil95/relict/assets/33003253/b22088b3-ec7e-42c2-b358-030815c7ab5a)

Implement the `Chromosome` trait:
https://github.com/vdwemil95/relict/blob/b920a008a02a83a89ba6f3d2dd687ce1d46c7838/examples/point_evolution.rs#L43-L54

Main loop:
https://github.com/vdwemil95/relict/blob/b920a008a02a83a89ba6f3d2dd687ce1d46c7838/examples/point_evolution.rs#L56-L99
