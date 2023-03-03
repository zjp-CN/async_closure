
Details:

| component   | rule / usage                                                               | trick / caveat                           |
|-------------|----------------------------------------------------------------------------|------------------------------------------|
| field_name  | used as unnamed struct's fields and variable names in async code           | rebind when mutation is required         |
| field_type  | owned types or referenced types                                            | only `'static` or `'a`  lifetime allowed |
| field_value | any value expression as the initial states / captured variables            |                                          |
| arg_name    | used as variable names in async code (`_` is not an ident thus disallowed) | rebind when mutation is required         |
| arg_type    | owned types or referenced types (normally no need to annotate lifetimes )  | `'a` is disallowed; but `'_` is allowed  |
| return_type | a must (even when `()` is returned)                                        |                                          |

Also note: arg_types and return_type must correspond to its `AsyncFn*` generic types, i.e.


