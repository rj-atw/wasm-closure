use wasmer_runtime::{imports, instantiate, Func, Array, WasmPtr, Instance, Memory};
use wasmer_runtime::units::Pages;
use wasmer_runtime::types::MemoryDescriptor;

use jni::JNIEnv;
use jni::objects::{JObject, JByteBuffer};
use jni::sys::{jint, jlong};

use std::convert::TryInto;

use wasmer_llvm_backend::LLVMCompiler;


#[no_mangle]
pub extern "system" fn Java_org_apache_spark_sql_catalyst_expressions_aggregate_WasmAggregation_reduce(
  env: JNIEnv, jself: JObject, 
  jProgram: JByteBuffer, jProgramLength: jint, 
  jInput: JByteBuffer, jInputLength: jint) -> jint {

    // TODO: Test for endianness

   //get data out of the ByteBuffers
   let program = &env.get_direct_buffer_address(jProgram).unwrap()[0..jProgramLength as usize];
   let input = &env.get_direct_buffer_address(jInput).unwrap()[0..jInputLength as usize];
   let input_len = (input.len()/8) as u32; 
   
   
    
   // Our import object, that allows exposing functions to our Wasm module.
   // We're not importing anything, so make an empty import object.
   let import_object = imports! {};

   let module = wasmer_runtime::compile_with(program, &LLVMCompiler::new()).expect("LLVM backend");
    // Let's create an instance of Wasm module running in the wasmer-runtime
   let mut instance = module.instantiate(&import_object).unwrap();

   let malloc: Func<u32, WasmPtr<u32,Array>> = instance.func("__wbindgen_malloc").expect("Invalid function");

   let wasm_ptr = malloc.call(input_len * 4).expect("WASM malloc call failed");
//     let wasm_ptr : WasmPtr<u32,Array> = WasmPtr::new(16);
    
 
   let mut sum = 0;
   
   unsafe {
      let wasm_array = wasm_ptr.deref_mut(instance.context().memory(0),0,input_len).unwrap();

       for idx in 0..(input_len as usize) {
         // Can look into using https://github.com/rust-lang/rust/issues/65807 once stable
         //wasm_array[idx].set(input[idx])
         let bytes = &input[idx*8..idx*8+8];
         let value = i64::from_be_bytes(bytes.try_into().expect("wrong length"));
         wasm_array[idx].set(value as u32);
         sum += value as u32;
       }
    }

    let reduce: Func<(WasmPtr<u32,Array>, u32), u32> =  instance.func("reduce").unwrap();
    let result = reduce.call(wasm_ptr, input_len).unwrap() as jint;

    result
}
