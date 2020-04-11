// Import the wasmer runtime so we can use it
use wasmer_runtime::{error, imports, instantiate, Func, Array, WasmPtr};
use std::fs::File;
use std::io::Read;
use std::env;

// Our entry point to our application
fn main() -> error::Result<()> {
   let args: Vec<String> = env::args().collect();

   let filepath = &args[1];
   let input: Vec<u32>  = args[2].split(",").map(|snum| snum.parse::<u32>().unwrap()).collect();

   let mut f = File::open(filepath).unwrap();
   let mut buffer = Vec::new();

    
    let num = f.read_to_end(&mut buffer).unwrap();

    // Our import object, that allows exposing functions to our Wasm module.
    // We're not importing anything, so make an empty import object.
    let import_object = imports! {};

    // Let's create an instance of Wasm module running in the wasmer-runtime
    let instance = instantiate(&buffer, &import_object)?;

    //let malloc: Func<u32, WasmPtr<u32, Array>> = instance.func("__wbindgen_malloc")?;
    let malloc: Func<u32, WasmPtr<u32,Array>> = instance.func("__wbindgen_malloc")?;

    let wasm_ptr = malloc.call(input.len() as u32 * 4)?;
   unsafe {
       let wasm_array = wasm_ptr.deref_mut(instance.context().memory(0),0,input.len() as u32).unwrap();
       for idx in 0..input.len() {
         wasm_array[idx].set(input[idx]);
       }
    //  let ( _, view ) =
      //  instance.context_mut().memory_and_data_mut::<&mut[u32;4]>(0);//wasm_ptr.offset());
      //view.get(0..r.len()) = r;
    }


    let reduce: Func<(WasmPtr<u32,Array>, u32), u32> =  instance.func("reduce")?;
    let result = reduce.call(wasm_ptr, input.len() as u32)?;


    // Log the new value
    println!("Result: {}", result);

    // Return OK since everything executed successfully!
    Ok(())
}
