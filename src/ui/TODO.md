## Go back to geometry shader idea

Everything until now has been about drawing quads. Can we combine all quads into one huge draw call?

To get there:

1. Make one unified shader that is always used for all UI stuff, using the formula `A * a + B * b * t1 + C * c * t2 + D * b * msdf(t1)` where `a`, `b` and `c` are colors, `t1` and `t2` are texture samplers, `msdf(t1)` is an MSDF calculator, and `A`, `B`, `C` and `D` are booleans derived from a mode parameter `m`.

Maybe a bit too complicated, we don't need two textures and MSDF should get phased out in favor of a regular atlas, so it becomes `a + b * t`, where `a` and `b` are colors, and `t` is an RGBA texture sampler. A subsequent parameter `m` denotes if `t` is sampled as RGBA or if only R, G, B or A are used as monochrome channels. A last parameter `f` indicates in which frame this rectangle should be rendered (see further).

2. Collect the uniforms into 20 per-vertex parameters:

    * `r.o.x`, `r.o.y`, `r.s.x`, `r.s.y`
    * `a.r`, `a.g`, `a.b`, `a.a`
    * `b.r`, `b.g`, `b.b`, `b.a`
    * `t.o.s`, `t.o.t`, `t.s.s`, `t.s.t`
    * `m`, `t.r`, `f`, reserved.

3. One draw pass now generates a bunch of rectangles that can be encoded as 20-parameter vertices, that expand into triangle fans by the geometry shader. Frames are managed as uniform arrays. So the only thing to push up to the GPU is changes in this uniform array, and vertex data that is not constant.
