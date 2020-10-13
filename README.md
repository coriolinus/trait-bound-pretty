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
