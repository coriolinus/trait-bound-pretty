# `trait-bound-pretty`

Occasionally, rust hands you errors like this:

```text
error[E0277]: the trait bound `Arc<sc_service::client::Client<sc_client_db::Backend<sp_runtime::generic::Block<sp_runtime::generic::Header<u32, BlakeTwo256>, OpaqueExtrinsic>>, LocalCallExecutor<sc_client_db::Backend<sp_runtime::generic::Block<sp_runtime::generic::Header<u32, BlakeTwo256>, OpaqueExtrinsic>>, NativeExecutor<Executor>>, sp_runtime::generic::Block<sp_runtime::generic::Header<u32, BlakeTwo256>, OpaqueExtrinsic>, RuntimeApi>>: HeaderBackend<sp_runtime::generic::Block<sp_runtime::generic::Header<u32, BlakeTwo256>, OpaqueExtrinsic>>` is not satisfied
```

This is hard for humans to read. It's somewhat easier if we unpack it into a tree structure:

```text
error[E0277]: the item:
  Arc<
    sc_service::client::Client<
      sc_client_db::Backend<
        sp_runtime::generic::Block<
          sp_runtime::generic::Header<
            u32,
            BlakeTwo256
          >,
          OpaqueExtrinsic
        >
      >,
      LocalCallExecutor<
        sc_client_db::Backend<
          sp_runtime::generic::Block<
            sp_runtime::generic::Header<
              u32,
              BlakeTwo256
            >,
            OpaqueExtrinsic
          >
        >,
        NativeExecutor<
          Executor
        >
      >,
      sp_runtime::generic::Block<
        sp_runtime::generic::Header<
          u32,
          BlakeTwo256
        >,
        OpaqueExtrinsic
      >,
      RuntimeApi
    >
  >
does not satisfy the trait bound:
  HeaderBackend<
    sp_runtime::generic::Block<
      sp_runtime::generic::Header<
        u32,
        BlakeTwo256
      >,
      OpaqueExtrinsic
    >
  >
```

That's what this does.

## Usage

This executable is a line-oriented stream editor, meaning that you can just feed data through it.
If it sees a line which can be parsed as an E0277 or as a type name, it will pretty-print it.
Otherwise, it just passes the data through unchanged.

### Example: check the current project, expanding E0277

```bash
cargo check 2> >(trait-bound-pretty)
```

### Usage Documentation

```text
USAGE:
    trait-bound-pretty [FLAGS]

FLAGS:
    -b, --bare-item
            Attempt to parse and print bare items instead of E0277 lines

    -f, --fail-fast
            If any parse error is produced, abort instead of continuing at the next line

    -h, --help
            Prints help information

    -s, --strict
            Activate strict mode

            Normally, any line of input which can't be parsed is passed through unchanged. In strict mode, any line of
            input which can't be parsed as an E0277 or a Rust type produces an error.
    -V, --version
            Prints version information

```
