// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Builtin interfaces: time, random numbers and networking. Every statement naming one of these modules lives
//! here, in the order the tables were written, so the module's merged interface keeps its copies'
//! order (row 648).

use crate::iface_chirho::builtin_chirho::package_exposure_chirho;
use crate::iface_chirho::builtin_chirho::{mk_type_chirho, mk_val_chirho};
use crate::iface_chirho::{IfaceExportsChirho, ModuleIfaceChirho};

/// Push the interfaces for time, random numbers and networking.
pub(super) fn push_time_network_chirho(modules_chirho: &mut Vec<ModuleIfaceChirho>) {
    // Network.HTTP.Types (http-types package — needed by servant, warp, etc.)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "statusCode",
            "statusMessage",
            "status200",
            "status301",
            "status302",
            "status400",
            "status401",
            "status403",
            "status404",
            "status500",
            "ok200",
            "notFound404",
            "methodGet",
            "methodPost",
            "methodPut",
            "methodDelete",
            "methodPatch",
            "methodHead",
            "methodOptions",
            "hContentType",
            "hAccept",
            "hAuthorization",
            "hCacheControl",
            "hCookie",
            "hContentLength",
            "parseQuery",
            "renderQuery",
            "simpleQueryToQuery",
            "queryToQueryBS",
            "urlDecode",
            "urlEncode",
            "decodePath",
            "encodePath",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for ty_chirho in &[
            ("Status", &["Status"][..]),
            ("HttpVersion", &["HttpVersion"][..]),
            ("Method", &[]),
            ("Header", &[]),
            ("HeaderName", &[]),
            ("ResponseHeaders", &[]),
            ("Query", &[]),
            ("QueryItem", &[]),
            (
                "StdMethod",
                &["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"],
            ),
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(ty_chirho.0, ty_chirho.1);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for mod_name_chirho in &[
            "Network.HTTP.Types",
            "Network.HTTP.Types.Status",
            "Network.HTTP.Types.Method",
            "Network.HTTP.Types.Header",
            "Network.HTTP.Types.URI",
            "Network.HTTP.Types.Version",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: None,
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Data.Time.Clock.POSIX
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "getPOSIXTime",
            "getCurrentTime",
            "posixSecondsToUTCTime",
            "utcTimeToPOSIXSeconds",
            "posixDayLength",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("POSIXTime", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Clock.POSIX".to_string(),
            exports_chirho,
        });
    }

    // Data.Time.Clock
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "getCurrentTime",
            "diffUTCTime",
            "addUTCTime",
            "secondsToNominalDiffTime",
            "nominalDiffTimeToSeconds",
            "nominalDay",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["UTCTime", "NominalDiffTime", "DiffTime"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Clock".to_string(),
            exports_chirho,
        });
    }

    // Data.Time.Calendar
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Day",
            "addDays",
            "diffDays",
            "toGregorian",
            "fromGregorian",
            "showGregorian",
            "isLeapYear",
            "addGregorianMonthsClip",
            "addGregorianYearsClip",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Day", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Calendar".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Calendar.OrdinalDate".to_string(),
            exports_chirho,
        });
    }

    // Data.Time.Calendar.Month
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Month",
            "addMonths",
            "subMonths",
            "diffMonths",
            "pattern",
            "MonthOfYear",
            "dayOfMonth",
            "fromMonthAndDay",
            "fromYearAndMonth",
            "monthToDay",
            "dayToMonth",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("Month", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Calendar.Month".to_string(),
            exports_chirho,
        });
    }

    // Data.Time.Calendar.Quarter
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "Quarter",
            "QuarterOfYear",
            "Q1",
            "Q2",
            "Q3",
            "Q4",
            "addQuarters",
            "diffQuarters",
            "dayOfQuarter",
            "fromQuarterAndDay",
            "fromYearAndQuarter",
            "quarterToDay",
            "dayToQuarter",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Quarter", "QuarterOfYear"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &["Q1", "Q2", "Q3", "Q4"]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Calendar.Quarter".to_string(),
            exports_chirho,
        });
    }

    // Data.Time.Calendar.MonthDay + WeekDate
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "monthAndDayToDayOfYear",
            "dayOfYearToMonthAndDay",
            "monthLength",
            "monthAndDayToDayOfYearValid",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Calendar.MonthDay".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "toWeekDate",
            "fromWeekDate",
            "fromWeekDateValid",
            "showWeekDate",
            "toWeekCalendarDate",
            "fromWeekCalendarDate",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Calendar.WeekDate".to_string(),
            exports_chirho,
        });
    }

    // Data.Time (umbrella re-export module)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "UTCTime",
            "NominalDiffTime",
            "DiffTime",
            "Day",
            "TimeOfDay",
            "LocalTime",
            "ZonedTime",
            "TimeZone",
            "UniversalTime",
            "TimeLocale",
            "FormatTime",
            "ParseTime",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "addUTCTime",
            "diffUTCTime",
            "getCurrentTime",
            "toGregorian",
            "fromGregorian",
            "addDays",
            "diffDays",
            "utc",
            "getTimeZone",
            "getCurrentTimeZone",
            "formatTime",
            "parseTimeM",
            "parseTimeOrError",
            "defaultTimeLocale",
            "readSTime",
            "readPTime",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Format".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Format.ISO8601".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Data.Time.Format.Internal".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.LocalTime".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
    }

    // Data.Time.Clock.TAI
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "AbsoluteTime",
            "taiEpoch",
            "taiClock",
            "addAbsoluteTime",
            "diffAbsoluteTime",
            "utcToTAITime",
            "taiToUTCTime",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Clock.TAI".to_string(),
            exports_chirho,
        });
    }

    // Data.Time.Clock.System
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "SystemTime",
            "systemEpochDay",
            "getSystemTime",
            "systemToUTCTime",
            "utcToSystemTime",
            "systemToTAITime",
            "truncateSystemTimeLeapSecond",
            "systemSeconds",
            "systemNanoseconds",
        ] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Clock.System".to_string(),
            exports_chirho,
        });
    }

    // Data.Time.Calendar.Easter
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["easterDayGregorian", "orthodoxEasterDay"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: package_exposure_chirho(&["time"]),
            name_chirho: "Data.Time.Calendar.Easter".to_string(),
            exports_chirho,
        });
    }

    // Data.Time.Calendar.Julian and other time sub-modules
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "toJulianYearAndDay",
            "fromJulianYearAndDay",
            "isJulianLeapYear",
            "julianMonthLength",
            "toJulian",
            "fromJulian",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for (mod_name_chirho, exposure_chirho) in &[
            (
                "Data.Time.Calendar.Julian",
                package_exposure_chirho(&["time"]),
            ),
            ("Data.Time.Calendar.Julian.YearDay", None),
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: exposure_chirho.clone(),
                name_chirho: mod_name_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }

    // Control.AutoUpdate (auto-update — needed by warp)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "defaultUpdateSettings",
            "mkAutoUpdate",
            "mkAutoUpdateWithModify",
            "updateAction",
            "updateFreq",
            "updateThreadName",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("UpdateSettings", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Control.AutoUpdate".to_string(),
            exports_chirho,
        });
    }

    // Control.Reaper (auto-update — needed by warp)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "mkReaper",
            "defaultReaperSettings",
            "reaperAdd",
            "reaperRead",
            "reaperStop",
            "reaperKill",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["ReaperSettings", "Reaper"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Control.Reaper".to_string(),
            exports_chirho,
        });
    }

    // Network.HTTP.Date (http-date — needed by warp)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "parseHTTPDate",
            "formatHTTPDate",
            "epochTimeToHTTPDate",
            "httpDateToUTC",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("HTTPDate", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Network.HTTP.Date".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["sendAll", "sendMany"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Network.Socket.ByteString".to_string(),
            exports_chirho,
        });
    }

    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["copy", "newBufferPool", "receive"] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["Buffer", "BufferPool", "BufSize"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Network.Socket.BufferPool".to_string(),
            exports_chirho,
        });
    }

    // Network.Socket (network — needed by warp)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "socket",
            "bind",
            "listen",
            "accept",
            "connect",
            "close",
            "getAddrInfo",
            "defaultHints",
            "setSocketOption",
            "withSocketsDo",
            "gracefulClose",
            "fdSocket",
            "socketToHandle",
            "getPeerName",
            "getSocketName",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "Socket",
            "SockAddr",
            "AddrInfo",
            "Family",
            "SocketType",
            "SocketOption",
            "HostAddress",
            "HostAddress6",
            "PortNumber",
        ] {
            let (k_chirho, v_chirho) = match *name_chirho {
                "SockAddr" => mk_type_chirho("SockAddr", &["SockAddrInet", "SockAddrInet6"]),
                "SocketOption" => mk_type_chirho("SocketOption", &["NoDelay"]),
                _ => mk_type_chirho(name_chirho, &[]),
            };
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Network.Socket".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "Network.Socket.Internal".to_string(),
            exports_chirho,
        });
    }

    // System.Random (random package — needed by QuickCheck)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        let (k_chirho, v_chirho) = mk_type_chirho("RandomGen", &["split", "genRange", "next"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("SplitGen", &["splitGen"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("StdGen", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) =
            mk_type_chirho("Random", &["random", "randomR", "randomRs", "randoms"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("Uniform", &["uniformM"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("UniformRange", &["uniformRM"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        let (k_chirho, v_chirho) = mk_type_chirho("StatefulGen", &[]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for name_chirho in &[
            "split",
            "splitGen",
            "next",
            "genRange",
            "mkStdGen",
            "newStdGen",
            "random",
            "randomR",
            "randomIO",
            "randomRIO",
            "randoms",
            "randomRs",
            "getStdRandom",
            "getStdGen",
            "setStdGen",
            "uniformM",
            "uniformRM",
            "uniformByteArray",
            "genWord8",
            "genWord16",
            "genWord32",
            "genWord64",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "System.Random".to_string(),
            exports_chirho,
        });
    }

    // System.Random.Stateful (random package — needed by QuickCheck)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &["StatefulGen", "FrozenGen", "RandomGenM", "MutableGen"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &["STGenM", "IOGenM", "AtomicGenM", "StateGenM"] {
            let (k_chirho, v_chirho) = mk_type_chirho(name_chirho, &[]);
            exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        }
        for name_chirho in &[
            "uniformM",
            "uniformRM",
            "uniformByteArray",
            "uniformByteStringM",
            "uniformShortByteString",
            "uniformWord8",
            "uniformWord16",
            "uniformWord32",
            "uniformWord64",
            "applySTGen",
            "applyIOGen",
            "applyAtomicGen",
            "newIOGenM",
            "newAtomicGenM",
            "newSTGenM",
            "withMutableGen",
            "withMutableGen_",
            "randomM",
            "randomRM",
            "splitGenM",
            "runStateGen",
            "runStateGen_",
            "runStateGenT",
            "runStateGenT_",
            "runStateGenST",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "System.Random.Stateful".to_string(),
            exports_chirho: exports_chirho.clone(),
        });
        modules_chirho.push(ModuleIfaceChirho {
            exposure_chirho: None,
            name_chirho: "System.Random.Internal".to_string(),
            exports_chirho,
        });
    }

    // System.Random.SplitMix (splitmix package — needed by random/QuickCheck)
    {
        let mut exports_chirho = IfaceExportsChirho::default();
        for name_chirho in &[
            "newSMGen",
            "initSMGen",
            "mkSMGen",
            "seedSMGen",
            "seedSMGen'",
            "unseedSMGen",
            "splitSMGen",
            "nextWord64",
            "nextWord32",
            "nextTwoWord32",
            "nextInt",
            "nextDouble",
            "nextFloat",
            "bitmaskWithRejection32",
            "bitmaskWithRejection64",
        ] {
            let (k_chirho, v_chirho) = mk_val_chirho(name_chirho);
            exports_chirho.values_chirho.insert(k_chirho, v_chirho);
        }
        let (k_chirho, v_chirho) = mk_type_chirho("SMGen", &["SMGen"]);
        exports_chirho.types_chirho.insert(k_chirho, v_chirho);
        for mod_chirho in &[
            "System.Random.SplitMix",
            "System.Random.SplitMix.Init",
            "System.Random.SplitMix32",
        ] {
            modules_chirho.push(ModuleIfaceChirho {
                exposure_chirho: None,
                name_chirho: mod_chirho.to_string(),
                exports_chirho: exports_chirho.clone(),
            });
        }
    }
}
