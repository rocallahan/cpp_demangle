#![allow(non_snake_case)]

extern crate cpp_demangle;
extern crate diff;

use std::io::Write;

fn assert_demangles_as(mangled: &str, expected: &str) {
    let sym = cpp_demangle::BorrowedSymbol::new(mangled.as_bytes())
        .expect("should parse mangled symbol ok");

    let mut actual = vec![];
    write!(&mut actual, "{}", sym).expect("should demangle symbol ok");
    let actual = String::from_utf8(actual).expect("should demangle to valid utf-8");

    if expected != actual {
        println!("");
        println!("Diff:");
        println!("--- expected");
        print!("+++ actual");

        let mut last = None;
        for cmp in diff::chars(expected, &actual) {
            match (last, cmp.clone()) {
                (Some(diff::Result::Left(_)), diff::Result::Left(_)) |
                (Some(diff::Result::Both(..)), diff::Result::Both(..)) |
                (Some(diff::Result::Right(_)), diff::Result::Right(_)) => {}

                (_, diff::Result::Left(_)) => print!("\n-"),
                (_, diff::Result::Both(..)) => print!("\n "),
                (_, diff::Result::Right(_)) => print!("\n+"),
            };
            match cmp.clone() {
                diff::Result::Left(c) |
                diff::Result::Both(c, _) |
                diff::Result::Right(c) => print!("{}", c),
            }
            last = Some(cmp);
        }
        println!("");
    }

    assert_eq!(expected, actual);
}

macro_rules! demangles {
    ( $mangled:ident , $demangled:expr ) => {
        #[test]
        fn $mangled() {
            assert_demangles_as(stringify!($mangled), $demangled);
        }
    }
}

demangles!(
    _Z20instantiate_with_intI3FooET_IiEv,
    "Foo<int> instantiate_with_int<Foo>()"
);
demangles!(_Z3fooISt6vectorIiEEvv, "void foo<std::vector<int> >()");
demangles!(__ZN3foo3barE3quxS0_, "foo::bar(qux, qux)");
demangles!(__ZN3foo3barE3quxS_, "foo::bar(qux, foo)");

demangles!(
    _ZN4funcI2TyEEN6ResultIT_EES3_,
    "Result<Ty> func<Ty>(Result<Ty>)"
);
demangles!(_ZN4funcI2TyEEN6ResultIT_EES2_, "Result<Ty> func<Ty>(Ty)");
demangles!(
    _ZN4funcI2TyEEN6ResultIT_EES1_,
    "Result<Ty> func<Ty>(Result)"
);
demangles!(_ZN4funcI2TyEEN6ResultIT_EES0_, "Result<Ty> func<Ty>(Ty)");
demangles!(_ZN4funcI2TyEEN6ResultIT_EES_, "Result<Ty> func<Ty>(func)");

demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), Ty)"
);
demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES0_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), Ty::method)"
);
demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES1_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), Ty)"
);
demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES2_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), char const)"
);
demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES3_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), char const*)"
);
demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES4_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), void (char const*))"
);
demangles!(
    _ZN2Ty6methodIS_EEvMT_FvPKcES5_,
    "void Ty::method<Ty>(void (Ty::*)(char const*), void (Ty::*)(char const*))"
);
demangles!(_ZNK1fB5cxx11Ev,"f[abi:cxx11]() const");
demangles!(
    _ZN4base8internal14CheckedSubImplIlEENSt9enable_ifIXsrSt14numeric_limitsIT_E10is_integerEbE4typeES4_S4_PS4_,
    "std::enable_if<std::numeric_limits<long>::is_integer, bool>::type base::internal::CheckedSubImpl<long>(long, long, long*)"
);

// Test cases found via differential testing against `c++filt` with `cargo-fuzz`
// and `libFuzzer`.

demangles!(
    _Z5ccc_Z5cccmmmml,
    "ccc_Z(cccmm, unsigned long, unsigned long, long)"
);
demangles!(
    __Z3S_Z3SGffffjjjjjjjjjjzjjjjjjojjjjjjjj,
    "S_Z(SGf, float, float, float, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, ..., unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned __int128, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int, unsigned int)"
);
demangles!(
    __Z3SGfDdedddd,
    "SGf(decimal64, long double, double, double, double, double)"
);
demangles!(
    __ZN6ISiS_Z3b_dE1ES0_7__dIFFFdhl,
    "ISiS_Z::b_d(E, E, __dIFFF, double, unsigned char, long)"
);
