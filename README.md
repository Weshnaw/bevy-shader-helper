A WIP tool to quickly generate shader boiler plate for bevy

Inspired from [prost-build](https://github.com/tokio-rs/prost), and [bevy_easy_compute](https://github.com/AnthonyTornetta/bevy_easy_compute)

# TODOs
## TODOs Short Term
- [ ] Implement Readable/Writable Buffer traits
- [ ] Implement texture_details
- [ ] Add a way to share buffers
- [ ] Derive macro should have a 'uniform' buffer attribute (relavent to buffer_entries function)

## TODOs Medium Term
- [ ] Cleanup the imports to use the more specific ones instead of the entirety of bevy
- [ ] Cleanup re-exports to be be more descriptive
- [ ] Doc comments...
- [ ] github actions...
- [ ] better logging...
- [ ] improve macros by restricting fields
- [ ] can I implement build such that it avoids using the derive macros?

## TODOs Long Term
- [ ] Allow for more dynamic shaders (i.e. Bevy's GOL example where they swap the image buffers)
- [ ] Implement build
    - [ ] Handle Entries (Only Compute for now)
    - [ ] Handle Buffers
    - [ ] Generate valid rust file
    - [ ] Insert rust file into the proper directory
    - [ ] Create macro to import generatedgi rust file
- [ ] Allow for Custom Shader Nodes
- [ ] Implement some system for other types of shaders (i.e. Vertex and Fragment shaders)
- [ ] Have someone who knows macros better refactor some of the less then optimal code
- [ ] More rigorous unit testing
- [ ] Better compiler errors
    - [ ] Maybe somehow have use functions from the macro tools

## TODO open issues
- [ ] rename BufferGroup::buffer_entries to be BufferGroup::bind_group_layout, and probably change get_bindings to bind_group_entries

## Unknown Feasibility
- [ ] Somehow combine ShaderBuffers and ShaderData
- [ ] More elagant solution to the texture_details; ideally one that would also de-dube the ShaderDataDetails::texture attribute fields
- [ ] Can I add compiler checks for entry enum; without requiring to pass the type down the plugin stack
