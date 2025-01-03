use crate::parse::Op;
use codegen::ir::UserFuncName;
use cranelift::{
    jit::{JITBuilder, JITModule},
    module::{default_libcall_names, Linkage, Module},
    prelude::*,
};
use std::mem;

pub fn jit(program: impl Iterator<Item = Op>) -> extern "C" fn(f64) -> f64 {
    let mut flag_builder = settings::builder();
    flag_builder.set("use_colocated_libcalls", "false").unwrap();
    // FIXME set back to true once the x64 backend supports it.
    flag_builder.set("is_pic", "false").unwrap();
    let isa_builder = cranelift::native::builder().unwrap_or_else(|msg| {
        panic!("host machine is not supported: {msg}");
    });
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .unwrap();
    let mut module = JITModule::new(JITBuilder::with_isa(isa, default_libcall_names()));

    let mut ctx = module.make_context();
    let mut func_ctx = FunctionBuilderContext::new();

    let mut sig = module.make_signature();
    sig.params.push(AbiParam::new(types::F64));
    sig.returns.push(AbiParam::new(types::F64));

    let func = module
        .declare_function("calculate", Linkage::Local, &sig)
        .unwrap();

    ctx.func.signature = sig;
    ctx.func.name = UserFuncName::user(0, func.as_u32());

    let mut fb = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
    let block = fb.create_block();
    fb.switch_to_block(block);
    fb.append_block_params_for_function_params(block);
    let one = fb.ins().f64const(1.0);
    let two = fb.ins().f64const(2.0);
    let mut value = fb.block_params(block)[0];
    for op in program {
        value = match op {
            Op::Add => fb.ins().fadd(value, one),
            Op::Sub => fb.ins().fsub(value, one),
            Op::Mul => fb.ins().fmul(value, two),
            Op::Div => fb.ins().fdiv(value, two),
        }
    }
    fb.ins().return_(&[value]);
    fb.seal_all_blocks();
    fb.finalize();

    module.define_function(func, &mut ctx).unwrap();
    module.clear_context(&mut ctx);

    module.finalize_definitions().unwrap();
    let code = module.get_finalized_function(func);
    unsafe { mem::transmute::<*const u8, extern "C" fn(f64) -> f64>(code) }
}
