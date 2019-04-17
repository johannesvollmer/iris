# Iris

Rust CPU ray tracer

My attempt at following [PBRT](http://pbr-book.org) in Rust. Heavily inspired by [tray_rust](https://github.com/Twinklebear/tray_rust), please check it out!

# Features
* Unbiased Monte Carlo Path Tracing
* Russian roulette
* BSDFs: Cook-Torrance Microfacet, Oren-Nayar, Lambert
* NDFs: Trowbridge-Reitz (GGX), Beckmann
* Materials: matte, mirror, plastic
* Lights: Area lights, point lights, spot lights
* Filters: Mitchell-Netravalli, triangle
* Tonemapping: Reinhard, Uncharted 2 Filmic
* OpenEXR and RGB16 output 

TODO:
* Generic Vec / Point with PhantomData over coordinate system
* Use TransformPair more
* Animated Transforms
* to_sin and to_cos methods on Float
* Fast intersections
* Window display