pub mod content_parser;

#[cfg(test)]

mod test {
    use crate::{
        backend::{Crates, Table, TableEntry},
        content_parser::content_parser::ContentParser,
    };

    async fn get_content_parser() -> ContentParser {
        let content_parser = ContentParser::new().await;

        content_parser
    }
    #[tokio::test]
    async fn should_get_networking_crates() {
        let networking_crates =  Table {
    entries: [
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "tokio".into(),
                    description: "The oldest async runtime in the Rust ecosystem and still the most widely supported. Recommended for new projects.".into(),
                    docs: "https://docs.rs/tokio/latest/tokio/".into(),
                },
                Crates {
                    name: "futures-executor".into(),
                    description: "A minimal executor. In particular, the block_on function is useful if you want to run an async function synchronously in codebase that is mostly synchronous.".into(),
                    docs: "https://docs.rs/futures-executor/latest/futures-executor/".into(),
                },
                Crates {
                    name: "futures".into(),
                    description: "Utility functions for working with Futures and Streams".into(),
                    docs: "https://docs.rs/futures/latest/futures/".into(),
                },
                Crates {
                    name: "async-trait".into(),
                    description: "Provides a workaround for the lack of language support for async functions in traits".into(),
                    docs: "https://docs.rs/async-trait/latest/async-trait/".into(),
                },
                Crates {
                    name: "glommio".into(),
                    description: "Use if you need io_uring support. Still somewhat experimental but rapidly maturing.".into(),
                    docs: "https://docs.rs/glommio/latest/glommio/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "http".into(),
                    description: "The `http` crate doesn't actually contain an HTTP implementation. Just types and interfaces to help interoperability.".into(),
                    docs: "https://docs.rs/http/latest/http/".into(),
                },
                Crates {
                    name: "hyper".into(),
                    description: "A low-level HTTP implementation (both client and server). Implements HTTP/1, and HTTP/2. Works best with the tokio async runtime, but can support other runtimes.".into(),
                    docs: "https://docs.rs/hyper/latest/hyper/".into(),
                },
                Crates {
                    name: "reqwest".into(),
                    description: "Full-fat HTTP client. Can be used in both synchronous and asynchronous code. Requires tokio runtime.".into(),
                    docs: "https://docs.rs/reqwest/latest/reqwest/".into(),
                },
                Crates {
                    name: "ureq".into(),
                    description: "Minimal synchronous HTTP client focussed on simplicity and minimising dependencies.".into(),
                    docs: "https://docs.rs/ureq/latest/ureq/".into(),
                },
                Crates {
                    name: "axum".into(),
                    description: "A minimal and ergonomic framework. An official part of the tokio project. Recommend for most new projects.".into(),
                    docs: "https://docs.rs/axum/latest/axum/".into(),
                },
                Crates {
                    name: "actix-web".into(),
                    description: "A performance focussed framework. All Rust frameworks are fast, but choose actix-web if you need the absolutely maximum performance.".into(),
                    docs: "https://docs.rs/actix-web/latest/actix-web/".into(),
                },
                Crates {
                    name: "async-graphql".into(),
                    description: "A high-performance graphql server library that's fully specification compliant. Integrates with actix-web, axum, poem, rocket, tide, warp.".into(),
                    docs: "https://docs.rs/async-graphql/latest/async-graphql/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "tungstenite".into(),
                    description: "Low-level crate that others build on".into(),
                    docs: "https://docs.rs/tungstenite/latest/tungstenite/".into(),
                },
                Crates {
                    name: "tokio-tungstenite".into(),
                    description: "If you are using the tokio executor".into(),
                    docs: "https://docs.rs/tokio-tungstenite/latest/tokio-tungstenite/".into(),
                },
                Crates {
                    name: "async-tungstenite".into(),
                    description: "If you are using the async-std executor".into(),
                    docs: "https://docs.rs/async-tungstenite/latest/async-tungstenite/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "tonic".into(),
                    description: "gRPC over HTTP/2 with full support for asynchronous code. Works with tokio".into(),
                    docs: "https://docs.rs/tonic/latest/tonic/".into(),
                },
            ].to_vec(),
        },
    ].to_vec(),


};

        let content_parser = get_content_parser().await;
        assert_eq!(
            networking_crates,
            content_parser
                .get_crates_with_sub(crate::backend::CategoriesWithSubCategories::Networking)
        );
    }

    #[tokio::test]
    async fn should_get_cryptography_crates() {
        let cryptography_crates = Table {
    entries: [
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "argon2".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/argon2/latest/argon2/".into(),
                },
                Crates {
                    name: "scrypt".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/scrypt/latest/scrypt/".into(),
                },
                Crates {
                    name: "bcrypt".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/bcrypt/latest/bcrypt/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "sha2".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/sha2/latest/sha2/".into(),
                },
                Crates {
                    name: "sha1".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/sha1/latest/sha1/".into(),
                },
                Crates {
                    name: "md-5".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/md-5/latest/md-5/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "aes-gcm-siv".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/aes-gcm-siv/latest/aes-gcm-siv/".into(),
                },
                Crates {
                    name: "aes-gcm".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/aes-gcm/latest/aes-gcm/".into(),
                },
                Crates {
                    name: "chacha20poly1305".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/chacha20poly1305/latest/chacha20poly1305/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "rsa".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/rsa/latest/rsa/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "ed25519".into(),
                    description: "Use in conjunction with the ed25519-dalek crate.".into(),
                    docs: "https://docs.rs/ed25519/latest/ed25519/".into(),
                },
                Crates {
                    name: "ecdsa".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/ecdsa/latest/ecdsa/".into(),
                },
                Crates {
                    name: "dsa".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/dsa/latest/dsa/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "der".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/der/latest/der/".into(),
                },
                Crates {
                    name: "pem-rfc7468".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/pem-rfc7468/latest/pem-rfc7468/".into(),
                },
                Crates {
                    name: "pkcs8".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/pkcs8/latest/pkcs8/".into(),
                },
                Crates {
                    name: "x509-cert".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/x509-cert/latest/x509-cert/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "rustls".into(),
                    description: "A portable pure-rust high-level implementation of TLS. Implements TLS 1.2 and higher.".into(),
                    docs: "https://docs.rs/rustls/latest/rustls/".into(),
                },
                Crates {
                    name: "native-tls".into(),
                    description: "Delegates to the system TLS implementations on windows and macOS, and uses OpenSSL on linux.".into(),
                    docs: "https://docs.rs/native-tls/latest/native-tls/".into(),
                },
                Crates {
                    name: "webpki".into(),
                    description: "X.509 Certificate validation. Builds on top of ring.".into(),
                    docs: "https://docs.rs/webpki/latest/webpki/".into(),
                },
                Crates {
                    name: "ring".into(),
                    description: "Fork of BoringSSL. Provides low-level cryptographic primitives for TLS/SSL.".into(),
                    docs: "https://docs.rs/ring/latest/ring/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "subtle".into(),
                    description: "Utilities for writing constant-time algorithms.".into(),
                    docs: "https://docs.rs/subtle/latest/subtle/".into(),
                },
                Crates {
                    name: "zeroize".into(),
                    description: "Securely erase memory.".into(),
                    docs: "https://docs.rs/zeroize/latest/zeroize/".into(),
                },
            ].to_vec(),
        },
    ].to_vec(),
};

        let content_parser = get_content_parser().await;

        assert_eq!(
            cryptography_crates,
            content_parser.get_crates(crate::backend::Categories::Cryptography)
        )
    }

    #[tokio::test]
    async fn should_get_math_crates() {
        let math_crate = Table {
            entries: [
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "num-traits".into(),
                            description: "Traits like Number, Add, etc that allow you write functions that are generic over the specific numeric type,".into(),
                            docs: "https://docs.rs/num-traits/latest/num-traits/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "num-bigint".into(),
                            description: "It's not the fastest, but it's part of the trusted num library.,".into(),
                            docs: "https://docs.rs/num-bigint/latest/num-bigint/".into(),
                        },
                        Crates {
                            name: "rug".into(),
                            description: "LGPL licensed. Wrapper for GMP. Much faster than num-bigint.,".into(),
                            docs: "https://docs.rs/rug/latest/rug/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "rust_decimal".into(),
                            description: "The binary representation consists of a 96 bit integer number, a scaling factor used to specify the decimal fraction and a 1 bit sign.,".into(),
                            docs: "https://docs.rs/rust_decimal/latest/rust_decimal/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "ordered-float".into(),
                            description: "Float types that don't allow NaN and are therefore orderable. You can also use the,total_cmp,method from the standard library like,.sort_by(|a, b| a.total_cmp(&b)),.,".into(),
                            docs: "https://docs.rs/ordered-float/latest/ordered-float/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "nalgebra".into(),
                            description: "General-purpose linear algebra library with transformations and statically-sized or dynamically-sized matrices. However it supports only vectors (1d) and matrices (2d) and not higher-dimensional tensors.,".into(),
                            docs: "https://docs.rs/nalgebra/latest/nalgebra/".into(),
                        },
                        Crates {
                            name: "ndarray".into(),
                            description: "Less featureful than nalgebra but supports arbitrarily dimensioned arrays,".into(),
                            docs: "https://docs.rs/ndarray/latest/ndarray/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "polars".into(),
                            description: "Similar to the Pandas library in Python but in pure Rust. Uses the Apache Arrow Columnar Format as the memory model.,".into(),
                            docs: "https://docs.rs/polars/latest/polars/".into(),
                        },
                        Crates {
                            name: "datafusion".into(),
                            description: "Apache DataFusion,is an in-memory query engine that uses Apache Arrow as the memory model,".into(),
                            docs: "https://docs.rs/datafusion/latest/datafusion/".into(),
                        },
                    ].to_vec(),
                },
            ].to_vec(),
        };

        let content_parser = get_content_parser().await;
        assert_eq!(
            math_crate,
            content_parser.get_crates(crate::backend::Categories::Math)
        );
    }

    #[tokio::test]
    async fn should_get_ffi_crates() {
        let ffi_crate = Table {
            entries: [
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "bindgen".into(),
                            description: "Generate Rust bindings to C libraries,".into(),
                            docs: "https://docs.rs/bindgen/latest/bindgen/".into(),
                        },
                        Crates {
                            name: "cbindgen".into(),
                            description: "Generate C bindings to Rust libraries,".into(),
                            docs: "https://docs.rs/cbindgen/latest/cbindgen/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "cxx".into(),
                            description: "Safe C++ <-> Rust interop by generating code for both sides.,".into(),
                            docs: "https://docs.rs/cxx/latest/cxx/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "pyo3".into(),
                            description: "Supports both calling python code from Rust and exposing Rust code to Python,".into(),
                            docs: "https://docs.rs/pyo3/latest/pyo3/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "napi".into(),
                            description: "is a framework for building pre-compiled Node.js addons in Rust.,".into(),
                            docs: "https://docs.rs/napi/latest/napi/".into(),
                        },
                        Crates {
                            name: "neon".into(),
                            description: "Slower than napi, but also widely used and well-maintained,".into(),
                            docs: "https://docs.rs/neon/latest/neon/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "rutie".into(),
                            description: "Supports both embedding Rust into Ruby applications and embedding Ruby into Rust applications,".into(),
                            docs: "https://docs.rs/rutie/latest/rutie/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "objc".into(),
                            description: "Interop with the Objective-C runtime,".into(),
                            docs: "https://docs.rs/objc/latest/objc/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "jni".into(),
                            description: "Implement Java methods for JVM and Android in Rust. Call Java code from Rust. Embed JVM in Rust applications.,".into(),
                            docs: "https://docs.rs/jni/latest/jni/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "mlua".into(),
                            description: "Bindings to Lua 5.4, 5.3, 5.2, 5.1 (including LuaJIT),".into(),
                            docs: "https://docs.rs/mlua/latest/mlua/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "flutter_rust_bridge".into(),
                            description: "Works with Dart with or without Flutter,".into(),
                            docs: "https://docs.rs/flutter_rust_bridge/latest/flutter_rust_bridge/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "rustler".into(),
                            description: "Safe Rust bridge for creating Erlang NIF functions,".into(),
                            docs: "https://docs.rs/rustler/latest/rustler/".into(),
                        },
                    ].to_vec(),
                },
            ].to_vec(),
        };

        let content_parser = get_content_parser().await;

        assert_eq!(
            ffi_crate,
            content_parser.get_crates(crate::backend::Categories::FFI)
        )
    }

    #[tokio::test]
    async fn should_get_general_crates() {
        let general_table : Table = Table {
            entries: [
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "rand".into(),
                            description: "De facto standard random number generation library split out from the standard library,".into(),
                            docs: "https://docs.rs/rand/latest/rand/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "time".into(),
                            description: "A smaller, simpler library. Preferrable if covers your needs, but it's quite limited in what it provides.,".into(),
                            docs: "https://docs.rs/time/latest/time/".into(),
                        },
                        Crates {
                            name: "chrono".into(),
                            description: "The most comprehensive and full-featured datetime library, but more complex because of it.,".into(),
                            docs: "https://docs.rs/chrono/latest/chrono/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "serde".into(),
                            description: "De facto standard serialization library. Use in conjunction with sub-crates like serde_json for the specific format that you are using.,".into(),
                            docs: "https://docs.rs/serde/latest/serde/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "regex".into(),
                            description: "De facto standard regex library. Very fast, but does not support fancier features such as backtracking.,".into(),
                            docs: "https://docs.rs/regex/latest/regex/".into(),
                        },
                        Crates {
                            name: "fancy-regex".into(),
                            description: "Use if need features such as backtracking which regex doesn't support,".into(),
                            docs: "https://docs.rs/fancy-regex/latest/fancy-regex/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "uuid".into(),
                            description: "Implements generating and parsing UUIDs and a number of utility functions,".into(),
                            docs: "https://docs.rs/uuid/latest/uuid/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "tempfile".into(),
                            description: "Supports both temporary files and temporary directories,".into(),
                            docs: "https://docs.rs/tempfile/latest/tempfile/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "flate2".into(),
                            description: "Uses a pure-Rust implementation by default. Use feature flags to opt in to system zlib.,".into(),
                            docs: "https://docs.rs/flate2/latest/flate2/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "indexmap".into(),
                            description: "A HashMap that seperately keeps track of insertion order and allows you to efficiently iterate over its elements in that order,".into(),
                            docs: "https://docs.rs/indexmap/latest/indexmap/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "arrayvec".into(),
                            description: "Arrays that are ONLY stack-allocated with fixed capacity,".into(),
                            docs: "https://docs.rs/arrayvec/latest/arrayvec/".into(),
                        },
                        Crates {
                            name: "smallvec".into(),
                            description: "Arrays that are stack-allocated with fallback to the heap if the fixed stack capacity is exceeded,".into(),
                            docs: "https://docs.rs/smallvec/latest/smallvec/".into(),
                        },
                        Crates {
                            name: "tinyvec".into(),
                            description: "Stack allocated arrays in 100% safe Rust code but requires items to implement the Default trait.,".into(),
                            docs: "https://docs.rs/tinyvec/latest/tinyvec/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "reqwest".into(),
                            description: "Full-fat HTTP client. Can be used in both synchronous and asynchronous code. Requires tokio runtime.,".into(),
                            docs: "https://docs.rs/reqwest/latest/reqwest/".into(),
                        },
                        Crates {
                            name: "ureq".into(),
                            description: "Minimal synchronous HTTP client focussed on simplicity and minimising dependencies.,".into(),
                            docs: "https://docs.rs/ureq/latest/ureq/".into(),
                        },
                    ].to_vec(),
                },
            ].to_vec(),
        };

        let content_parser = get_content_parser().await;

        assert_eq!(general_table, content_parser.get_general_crates());
    }

    #[tokio::test]
    async fn should_get_common_crates() {
        let common_crates = Table {
    entries: [
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "anyhow".into(),
                    description: "Provides a boxed error type that can hold any error, and helpers for generating an application-level stack trace.".trim().into(),
                    docs: "https://docs.rs/anyhow/latest/anyhow/".into(),
                },
                Crates {
                    name: "color-eyre".into(),
                    description: "A fork of anyhow that gives you more control over the format of the generated error messages. Recommended if you intend to present error messages to end users. Otherwise anyhow is simpler.".trim().into(),
                    docs: "https://docs.rs/color-eyre/latest/color-eyre/".into(),
                },
                Crates {
                    name: "thiserror".into(),
                    description: "Helps with generating boilerplate for enum-style error types.".into(),
                    docs: "https://docs.rs/thiserror/latest/thiserror/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "tracing".into(),
                    description: "Tracing is now the go-to crate for logging.".into(),
                    docs: "https://docs.rs/tracing/latest/tracing/".into(),
                },
                Crates {
                    name: "log".into(),
                    description: "An older and simpler crate if your needs are simple and you are not using any async code.".into(),
                    docs: "https://docs.rs/log/latest/log/".into(),
                },
                Crates {
                    name: "tracing".into(),
                    description: "Tracing is now the go-to crate for logging.".into(),
                    docs: "https://docs.rs/tracing/latest/tracing/".into(),
                },
                Crates {
                    name: "slog".into(),
                    description: "Structured logging".into(),
                    docs: "https://docs.rs/slog/latest/slog/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "once_cell".into(),
                    description: "Newer crate with more ergonomic API. Should be preferred for all new projects.".into(),
                    docs: "https://docs.rs/once_cell/latest/once_cell/".into(),
                },
                Crates {
                    name: "lazy_static".into(),
                    description: "Older crate. API is less convenient, but crate is stable and maintained.".into(),
                    docs: "https://docs.rs/lazy_static/latest/lazy_static/".into(),
                },
                Crates {
                    name: "itertools".into(),
                    description: "A bunch of useful methods on iterators that aren't in the stdlib".into(),
                    docs: "https://docs.rs/itertools/latest/itertools/".into(),
                },
                Crates {
                    name: "syn".into(),
                    description: "Parse rust source code".into(),
                    docs: "https://docs.rs/syn/latest/syn/".into(),
                },
                Crates {
                    name: "quote".into(),
                    description: "Quasi quoting rust (useful for interpolating generated code with literal code)".into(),
                    docs: "https://docs.rs/quote/latest/quote/".into(),
                },
                Crates {
                    name: "paste".into(),
                    description: "Concatenating and manipulating identifiers".into(),
                    docs: "https://docs.rs/paste/latest/paste/".into(),
                },
                Crates {
                    name: "bytemuck".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/bytemuck/latest/bytemuck/".into(),
                },
                Crates {
                    name: "zerocopy".into(),
                    description: "no description".into(),
                    docs: "https://docs.rs/zerocopy/latest/zerocopy/".into(),
                },
                Crates {
                    name: "bitflags".into(),
                    description: "Strongly typed bitflag types".into(),
                    docs: "https://docs.rs/bitflags/latest/bitflags/".into(),
                },
            ].to_vec(),
        },
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "memmap2".into(),
                    description: "The older memmap crate is unmaintained.".into(),
                    docs: "https://docs.rs/memmap2/latest/memmap2/".into(),
                },
                Crates {
                    name: "libc".into(),
                    description: "Bindings for directly calling libc functions.".into(),
                    docs: "https://docs.rs/libc/latest/libc/".into(),
                },
                Crates {
                    name: "windows".into(),
                    description: "The official Microsoft-provided crate for interacting with windows APIs".into(),
                    docs: "https://docs.rs/windows/latest/windows/".into(),
                },
                Crates {
                    name: "winapi".into(),
                    description: "Older binding to the windows APIs. Unofficial, but more complete than windows-rs".into(),
                    docs: "https://docs.rs/winapi/latest/winapi/".into(),
                },
                Crates {
                    name: "nix".into(),
                    description: "Bindings to the various *nix system functions. (Unix, Linux, MacOS, etc.)".into(),
                    docs: "https://docs.rs/nix/latest/nix/".into(),
                },
            ].to_vec(),
        },
    ].to_vec(),
};
        let content_parser = get_content_parser().await;
        assert_eq!(
            common_crates,
            content_parser.get_crates_with_sub(crate::backend::CategoriesWithSubCategories::Common)
        );
    }

    #[tokio::test]
    async fn should_get_concurrency_crates() {
        let concurrency_crates = Table {
    entries: [
        TableEntry {
            use_case: "".into(),
            crates: [
                Crates {
                    name: "parking_lot".into(),
                    description: "std::sync::Mutex also works fine. But Parking Lot is faster.".into(),
                    docs: "https://docs.rs/parking_lot/latest/parking_lot/".into(),
                },
                Crates {
                    name: "arc-swap".into(),
                    description: "Useful for sharing data that has many readers but few writers".into(),
                    docs: "https://docs.rs/arc-swap/latest/arc-swap/".into(),
                },
                Crates {
                    name: "dashmap".into(),
                    description: "The fastest for general purpose workloads".into(),
                    docs: "https://docs.rs/dashmap/latest/dashmap/".into(),
                },
                Crates {
                    name: "flurry".into(),
                    description: "Particularly good for read-heavy workloads.".into(),
                    docs: "https://docs.rs/flurry/latest/flurry/".into(),
                },
                Crates {
                    name: "crossbeam-channel".into(),
                    description: "The absolute fastest channel implementation available. Implements Go-like 'select' feature.".into(),
                    docs: "https://docs.rs/crossbeam-channel/latest/crossbeam-channel/".into(),
                },
                Crates {
                    name: "flume".into(),
                    description: "Smaller and simpler than crossbeam-channel and almost as fast".into(),
                    docs: "https://docs.rs/flume/latest/flume/".into(),
                },
                Crates {
                    name: "tokio".into(),
                    description: "Tokio's sync module provides channels for using in async code".into(),
                    docs: "https://docs.rs/tokio/latest/tokio/".into(),
                },
                Crates {
                    name: "postage".into(),
                    description: "Channels that integrate nicely with async code, with different options than Tokio".into(),
                    docs: "https://docs.rs/postage/latest/postage/".into(),
                },
                Crates {
                    name: "rayon".into(),
                    description: "Convert sequential computation into parallel computation with one call - `par_iter` instead of `iter`".into(),
                    docs: "https://docs.rs/rayon/latest/rayon/".into(),
                },
            ].to_vec(),
        },
    ].to_vec(),
};

        let content_parser = get_content_parser().await;
        assert_eq!(
            concurrency_crates,
            content_parser
                .get_crates_with_sub(crate::backend::CategoriesWithSubCategories::Concurrency)
        );
    }
}
