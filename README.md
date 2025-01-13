A WIP tool to quickly generate shader boiler plate for bevy

Inspired from [prost-build](https://github.com/tokio-rs/prost), and [bevy_easy_compute](https://github.com/AnthonyTornetta/bevy_easy_compute)

# TODOs
## TODOs Short Term
- [ ] Add a way to share buffers
- [ ] Derive macro should have a 'uniform' buffer attribute (relavent to buffer_entries function)

## TODOs Medium Term
- [ ] Cleanup re-exports to be be more descriptive
- [ ] Doc comments...
- [ ] github actions...
- [ ] better logging...
- [ ] improve macros by restricting fields

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
- [ ] More rigorous unit testing
- [ ] Better compiler errors
    - [ ] Maybe somehow have use functions from the macro tools

## TODO open issues
- [ ] rename BufferGroup::buffer_entries to be BufferGroup::bind_group_layout, and probably change get_bindings to bind_group_entries
