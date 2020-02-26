use std::ffi::CString;

use super::*;

pub struct CallBuilder;

impl CallBuilder {
    pub fn build<'f, 'o>(
        builder: &mut ScopedFunctionBuilder<'f, 'o>,
        ir_value: Option<ir::Value>,
        op: Call,
    ) -> Result<Option<Value>> {
        match op.callee {
            Callee::Static(ref ident) => {
                builder.debug(&format!("static call target is {}", ident));

                let is_local = builder.is_current_module(ident.module.name);
                let module = CString::new(ident.module.to_string()).unwrap();
                let name = CString::new(format!("{}/{}", ident.name, ident.arity)).unwrap();
                let args = op
                    .args
                    .iter()
                    .copied()
                    .map(|v| builder.value_ref(v))
                    .collect::<Vec<_>>();
                builder.debug(&format!("static call args: {:?}", args.as_slice()));
                let mut ok_args = Vec::new();
                let ok_block = match op.ok {
                    CallSuccess::Branch(Branch { block, args }) => {
                        for arg in args.iter().copied() {
                            ok_args.push(builder.value_ref(arg));
                        }
                        builder.block_ref(block)
                    }
                    _ => Default::default(),
                };
                builder.debug(&format!("static call ok args: {:?}", ok_args.as_slice()));
                let mut err_args = Vec::new();
                let err_block = match op.err {
                    CallError::Branch(Branch { block, args }) => {
                        for arg in args.iter().copied() {
                            err_args.push(builder.value_ref(arg));
                        }
                        builder.block_ref(block)
                    }
                    _ => Default::default(),
                };
                builder.debug(&format!("static call err args: {:?}", err_args.as_slice()));
                unsafe {
                    MLIRBuildStaticCall(
                        builder.as_ref(),
                        module.as_ptr(),
                        name.as_ptr(),
                        args.as_ptr(),
                        args.len() as libc::c_uint,
                        op.is_tail,
                        ok_block,
                        ok_args.as_ptr(),
                        ok_args.len() as libc::c_uint,
                        err_block,
                        err_args.as_ptr(),
                        err_args.len() as libc::c_uint,
                    );
                }

                Ok(None)
            }
            callee => todo!("unimplemented call type {:?}", callee),
        }
    }
}

pub struct CalleeBuilder;

impl CalleeBuilder {
    pub fn build<'f, 'o>(
        _builder: &mut ScopedFunctionBuilder<'f, 'o>,
        ir_value: Option<ir::Value>,
        callee: Callee,
    ) -> Result<Option<Value>> {
        todo!(
            "build function reference constant for {:?} = {:?}",
            ir_value,
            callee
        );
    }
}
