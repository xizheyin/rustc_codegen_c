# Rust to C Backend
This is an experimental rust2c backend.


# Code Structure

- `rgenc`: The backend to generate C.
    - `codegen`: The logic to convert MIR to C AST is implemented by impl Context.
    - `context`: Stores information about the context of the translation, including TyCtxt or some other global variable.
    - `backend_interface`: Implements the interface `impl CodegenBackend for ...` provided by rustc.
    - `util`: Some auxiliary functions.
- `rust2cast`: The C ast we use for codegen.
- `src`: The command line tool, which I was going to use to run the whole thing, including compiling, calling gcc, etc., but I don't really think about how much functionality there is at the moment, and it doesn't seem like I'll need it much for a while. If necessary, I can replace it with build_system, which will be much more streamlined.
