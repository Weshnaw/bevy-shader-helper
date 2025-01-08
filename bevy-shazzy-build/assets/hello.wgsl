struct Foo {
    bar: u32,
    baz: f32,
}

@group(0) @binding(0) var<storage, read_write> a: array<u32>;
@group(0) @binding(1) var<storage, read>       b: Foo;
@group(0) @binding(2) var<storage, read>       c: vec3<f32>;
@group(0) @binding(3) var<uniform>             d: u32;
@group(0) @binding(4) var                      e: texture_storage_2d<r32float, write>;



@compute @workgroup_size(1) fn main() {}