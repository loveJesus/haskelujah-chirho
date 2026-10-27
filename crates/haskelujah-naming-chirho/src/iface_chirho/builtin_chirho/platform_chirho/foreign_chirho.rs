// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: the foreign function interface. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for the foreign function interface.
pub(super) fn push_foreign_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Foreign.Ptr / Foreign.ForeignPtr / Foreign.Storable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["nullPtr", "plusPtr", "castPtr", "minusPtr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Ptr", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("FunPtr", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Ptr".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        let (k_chirho, v_chirho) = mk_type_chirho("ForeignPtr", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "newForeignPtr",
            "withForeignPtr",
            "unsafeWithForeignPtr",
            "mallocForeignPtrBytes",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.ForeignPtr".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign".to_string(),
            exports_chirho,
        });
        let mut stor_exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "sizeOf",
            "alignment",
            "peek",
            "poke",
            "peekByteOff",
            "pokeByteOff",
            "peekElemOff",
            "pokeElemOff",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            stor_exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Storable", &[]);
        stor_exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Storable".to_string(),
            exports_chirho: stor_exports_chirho,
        });
    }

    // Foreign / Foreign.Ptr / Foreign.C / Foreign.Storable / Foreign.ForeignPtr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Ptr",
            "FunPtr",
            "IntPtr",
            "WordPtr",
            "ForeignPtr",
            "StablePtr",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "nullPtr",
            "plusPtr",
            "castPtr",
            "alignPtr",
            "ptrToIntPtr",
            "intPtrToPtr",
            "peek",
            "poke",
            "peekByteOff",
            "pokeByteOff",
            "peekElemOff",
            "pokeElemOff",
            "sizeOf",
            "alignment",
            "malloc",
            "free",
            "alloca",
            "allocaBytes",
            "newForeignPtr",
            "withForeignPtr",
            "unsafeWithForeignPtr",
            "mallocForeignPtr",
            "newStablePtr",
            "deRefStablePtr",
            "freeStablePtr",
            "castStablePtrToPtr",
            "castPtrToStablePtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"Storable";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        // Push as multiple module names
        for mod_name_chirho in &[
            "Foreign",
            "Foreign.Ptr",
            "Foreign.ForeignPtr",
            "Foreign.Storable",
            "Foreign.StablePtr",
            "Foreign.Marshal",
            "Foreign.Marshal.Alloc",
            "Foreign.Marshal.Utils",
            "Foreign.Marshal.Pool",
            "Foreign.Marshal.Error",
            "Foreign.Marshal.Safe",
            "Foreign.Marshal.Array",
            "Foreign.C",
            "Foreign.C.Types",
            "Foreign.C.String",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // GHC.ForeignPtr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "ForeignPtr",
            "newForeignPtr",
            "newForeignPtr_",
            "withForeignPtr",
            "unsafeWithForeignPtr",
            "memset",
            "plusForeignPtr",
            "mallocPlainForeignPtrBytes",
            "castForeignPtr",
            "mallocForeignPtr",
            "FinalizerPtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ForeignPtr", "FinalizerPtr"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "GHC.ForeignPtr".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.ForeignPtr".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Ptr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Ptr",
            "nullPtr",
            "castPtr",
            "plusPtr",
            "alignPtr",
            "FunPtr",
            "nullFunPtr",
            "castFunPtr",
            "castFunPtrToPtr",
            "castPtrToFunPtr",
            "WordPtr",
            "IntPtr",
            "ptrToWordPtr",
            "wordPtrToPtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Ptr", "FunPtr", "WordPtr", "IntPtr"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Ptr".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Storable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Storable",
            "sizeOf",
            "alignment",
            "peek",
            "poke",
            "peekByteOff",
            "pokeByteOff",
            "peekElemOff",
            "pokeElemOff",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Storable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Storable".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Marshal.Alloc
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "malloc",
            "mallocBytes",
            "calloc",
            "callocBytes",
            "realloc",
            "reallocBytes",
            "free",
            "alloca",
            "allocaBytes",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Marshal.Alloc".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.Types
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CInt", "CUInt", "CLong", "CULong", "CChar", "CUChar", "CShort", "CUShort", "CFloat",
            "CDouble", "CSize", "CLLong", "CULLong", "CBool", "CWchar",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[name_chirho]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            let (k2_chirho, v2_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k2_chirho, v2_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.Types".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.String
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CString",
            "CStringLen",
            "newCString",
            "newCStringLen",
            "withCString",
            "withCStringLen",
            "peekCString",
            "peekCStringLen",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.String".to_string(),
            exports_chirho,
        });
    }

    // Foreign (umbrella module)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Ptr",
            "nullPtr",
            "castPtr",
            "plusPtr",
            "FunPtr",
            "nullFunPtr",
            "castFunPtr",
            "ForeignPtr",
            "newForeignPtr",
            "withForeignPtr",
            "Storable",
            "sizeOf",
            "alignment",
            "peek",
            "poke",
            "malloc",
            "free",
            "alloca",
            "CInt",
            "CUInt",
            "CLong",
            "CChar",
            "CDouble",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Storable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("Storable", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "sizeOf",
            "alignment",
            "peek",
            "poke",
            "peekElemOff",
            "pokeElemOff",
            "peekByteOff",
            "pokeByteOff",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Storable".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Ptr
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["Ptr", "FunPtr", "WordPtr", "IntPtr"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "nullPtr",
            "castPtr",
            "plusPtr",
            "alignPtr",
            "minusPtr",
            "nullFunPtr",
            "castFunPtr",
            "castFunPtrToPtr",
            "freeHaskellFunPtr",
            "ptrToWordPtr",
            "wordPtrToPtr",
            "ptrToIntPtr",
            "intPtrToPtr",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Ptr".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Storable
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "sizeOf",
            "alignment",
            "peek",
            "poke",
            "peekElemOff",
            "pokeElemOff",
            "peekByteOff",
            "pokeByteOff",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        {
            let name_chirho = &"Storable";
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Storable".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.ConstPtr (GHC 9.10+)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("ConstPtr", &["ConstPtr"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &["ConstPtr", "unConstPtr"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.ConstPtr".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.Types
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CInt",
            "CChar",
            "CShort",
            "CLong",
            "CLLong",
            "CUInt",
            "CUChar",
            "CUShort",
            "CULong",
            "CULLong",
            "CFloat",
            "CDouble",
            "CSize",
            "CSsize",
            "CPtrdiff",
            "CBool",
            "CWchar",
            "CSigAtomic",
            "CClock",
            "CTime",
            "CIntPtr",
            "CUIntPtr",
            "CIntMax",
            "CUIntMax",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[name_chirho]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.Types".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.String
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CString",
            "CStringLen",
            "peekCString",
            "peekCStringLen",
            "newCString",
            "newCStringLen",
            "withCString",
            "withCStringLen",
            "castCharToCChar",
            "castCCharToChar",
            "castCUCharToChar",
            "castCharToCUChar",
            "CWString",
            "CWStringLen",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.String".to_string(),
            exports_chirho,
        });
    }

    // Foreign.C.Error
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Errno",
            "eOK",
            "e2BIG",
            "eACCES",
            "eAGAIN",
            "eBADF",
            "eBUSY",
            "eCHILD",
            "eDEADLK",
            "eEXIST",
            "eFAULT",
            "eFBIG",
            "eINTR",
            "eINVAL",
            "eIO",
            "eISDIR",
            "eMFILE",
            "eMLINK",
            "eNAMETOOLONG",
            "eNFILE",
            "eNODEV",
            "eNOENT",
            "eNOEXEC",
            "eNOLCK",
            "eNOMEM",
            "eNOSPC",
            "eNOSYS",
            "eNOTDIR",
            "eNOTEMPTY",
            "eNOTTY",
            "eNXIO",
            "ePERM",
            "ePIPE",
            "eRANGE",
            "eROFS",
            "eSPIPE",
            "eSRCH",
            "eXDEV",
            "isValidErrno",
            "getErrno",
            "resetErrno",
            "throwErrno",
            "throwErrnoIf",
            "throwErrnoIf_",
            "throwErrnoIfRetry",
            "throwErrnoIfMinus1",
            "throwErrnoIfNull",
            "throwErrnoPath",
            "throwErrnoPathIf",
            "throwErrnoPathIfNull",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Errno", &["Errno"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C.Error".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Marshal.Alloc / Foreign.Marshal.Array / Foreign.Marshal.Utils
    for mod_name_chirho in &[
        "Foreign.Marshal.Alloc",
        "Foreign.Marshal.Array",
        "Foreign.Marshal.Utils",
        "Foreign.Marshal",
    ] {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "malloc",
            "mallocBytes",
            "alloca",
            "allocaBytes",
            "realloc",
            "reallocBytes",
            "free",
            "finalizerFree",
            "mallocArray",
            "mallocArray0",
            "allocaArray",
            "allocaArray0",
            "reallocArray",
            "reallocArray0",
            "peekArray",
            "peekArray0",
            "pokeArray",
            "pokeArray0",
            "newArray",
            "newArray0",
            "withArray",
            "withArray0",
            "withArrayLen",
            "withArrayLen0",
            "advancePtr",
            "copyArray",
            "moveArray",
            "lengthArray0",
            "new",
            "with",
            "fromBool",
            "toBool",
            "maybeNew",
            "maybeWith",
            "maybePeek",
            "withMany",
            "copyBytes",
            "moveBytes",
            "fillBytes",
            "memset",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: mod_name_chirho.to_string(),
            exports_chirho,
        });
    }

    // Foreign.C (umbrella)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "CInt",
            "CChar",
            "CShort",
            "CLong",
            "CUInt",
            "CFloat",
            "CDouble",
            "CSize",
            "CString",
            "CStringLen",
            "CWString",
            "Errno",
            "castCharToCChar",
            "castCCharToChar",
            "peekCString",
            "newCString",
            "withCString",
            "ConstPtr",
            "throwErrnoIfMinus1_",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.C".to_string(),
            exports_chirho,
        });
    }

    // Foreign.Concurrent
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["newForeignPtr", "addForeignPtrFinalizer"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.Concurrent".to_string(),
            exports_chirho,
        });
    }

    // Foreign.ForeignPtr.Unsafe
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        {
            let name_chirho = &"unsafeForeignPtrToPtr";
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            name_chirho: "Foreign.ForeignPtr.Unsafe".to_string(),
            exports_chirho,
        });
    }
}
