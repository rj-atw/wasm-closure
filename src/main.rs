// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, Func, Array, WasmPtr};
use wasmer_runtime::memory::MemoryView;

// Our entry point to our application
fn main() -> error::Result<()> {
    // Let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("add.wasm");

    // Our import object, that allows exposing functions to our Wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};

    // Let's create an instance of Wasm module running in the wasmer-runtime
    let mut instance = instantiate(wasm_bytes, &import_object)?;

    //let malloc: Func<u32, WasmPtr<u32, Array>> = instance.func("__wbindgen_malloc")?;
    let malloc: Func<u32, WasmPtr<u32,Array>> = instance.func("__wbindgen_malloc")?;
    let r = [1,2,3,4];

    let wasm_ptr = malloc.call(r.len() as u32 *4)?;
   unsafe {
    wasm_ptr.deref_mut(instance.context().memory(0),0,4).unwrap()[0].set(4) ;
    wasm_ptr.deref_mut(instance.context().memory(0),0,4).unwrap()[1].set(39) ;
    //  let ( _, view ) =
      //  instance.context_mut().memory_and_data_mut::<&mut[u32;4]>(0);//wasm_ptr.offset());
      //view.get(0..r.len()) = r;
    }


    let add: Func<(WasmPtr<u32,Array>, u32), u32> =  instance.func("add")?;
    let result = add.call(wasm_ptr, 4)?;


    // Log the new value
    println!("Result: {}", result);

    // Asserting that the returned value from the function is our expected value.
    assert_eq!(result, 43);

    // Return OK since everything executed successfully!
    Ok(())
}
