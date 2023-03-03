
# Syntax

Consisted of two parts:

1. a block `{};` in which multiple assignments `field_name: field_type = field_value` are seperated by `,`
2. an async closure `async |arg1: arg1_type, ...| -> return_type { /* any async code here */ }`

Fields or arguments can be empty, i.e.
* empty fields: `{};`
* empty arguments: `async | | -> return_type { /* async code and captured variables */ }`  
  (note it's `| |` instead of `||`, and the return_type is a must anyway)


