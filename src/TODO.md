# TODO

- move Session into System as default for graphics
- move SwapChain into Window
- call draw_begin() from Window, this gives back a command buffer reference, draw in that, then call draw_end() and present(), all on Window

GPU passes:
    atmosphere
    render depth only
    render opaque
    render decals
    render additive decals
    light shaft occlusion
    generate shadow views
    light probe lighting
    deferred light pass
    subsurface scattering
    ambient occlusion
    shading
    water and transparents
    lens flare and gunk
    postprocess
