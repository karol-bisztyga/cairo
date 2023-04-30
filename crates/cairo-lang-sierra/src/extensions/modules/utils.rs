use crate::extensions::lib_func::{
    LibfuncSignature, OutputVarInfo, ParamSignature, SierraApChange,
};
use crate::extensions::OutputVarReferenceInfo;
use crate::ids::ConcreteTypeId;

/// Returns a libfunc signature that casts from one type to another.
pub fn reinterpret_cast_signature(
    from_ty: ConcreteTypeId,
    to_ty: ConcreteTypeId,
) -> LibfuncSignature {
    LibfuncSignature::new_non_branch_ex(
        vec![ParamSignature::new(from_ty).allow_all()],
        vec![OutputVarInfo {
            ty: to_ty,
            ref_info: OutputVarReferenceInfo::SameAsParam { param_idx: 0 },
        }],
        SierraApChange::Known { new_vars_only: true },
    )
}
