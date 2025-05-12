# Open-RC2k

An attempt at reverse engineering and reconstructing the game in Rust.

# How to install

Drop both libmem.dll file and the compiled dinput.dll to the game folder, next to ral.exe.

Libmem.dll can be obtained from [here](https://github.com/rdbo/libmem/releases/tag/4.0.0) (Download libmem-win-x86.zip,
the DLL will be located in `\lib\libmem.dll`)

# Issues

That I know of, the game should run as if it was stock. Only difference that should be noticeable is a console that pops
up when the game opens.

# How to compile

- Install Rust from [here](https://www.rust-lang.org/tools/install).
    - When installing, select option 1 (to install through the VS Community Installer)
    - Then, option 2 to customize installation, and set the following values
        - Default host triple: `i686-pc-windows-msvc`
        - Default toolchain: `nightly`
        - Profile: leave default
        - Path variable: Y
- In a terminal, navigate to the project folder, and run the following:
    - `cargo build --release`
- This should output the created `dinput.dll` to the `\target\i686-pc-windows-msvc\release` folder on the project root.

# Code structure

## Argument types

Methods directly hooked will use raw pointers and data structures, but ideally multi level rewritten methods should use
Rust's native structures.
What this means is something like the following:

- Method `write_to_file(mut buffer: *mut u8)`
    - which is directly hooked, and as such takes the game's provided data raw, and so the parameter must be a raw
      pointer to a u8 (which would contain multiple u8, forming a raw C string)

- Method `log(mut buffer: *mut u8) -> write_to_file(buffer: String)`
    - `log` is directly hooked, and must take the raw pointer as well, but this time it calls `write_to_file`, so if
      possible to convert the raw pointer to an actual Rust String to pass to `write_to_file`, doing so is desirable

Methods are directly hooked when a line like the following exists in that module's `inject_hooks`:

```rust
let close_file_params_hk_addr = close_file_parameters as * const () as lm_address_t;
let _ = LM_HookCode(0x402E23, close_file_params_hk_addr).unwrap();
```

## Arguments obtainability

Rally Championship 2000 likes to pass it's arguments to functions in a somewhat unusual way, at least from what I've
seen.

Normally arguments are `push`ed to the stack before the function is called, and `pop`ed inside the function to obtain
them.

RC2k however (from what I've seen probably due to whole program optimization being enable when compiling) passes
arguments mostly on the registers, however it wants, with no real rhyme or reason.

Which means, instead of arguments being pushed sequentially to the stack, they're just passed in registers, and the only
way to know which registers is to analyse the disassembly.

This introduces a problem for us however, as Rust doesn't allow us to just get a method's arguments from wherever we
want, it has to follow some conventions.

As such, the way I found to fix this problem was to use unsafe(naked) methods (that don't create a prolog) that set the
stack how we want before calling the actual method.

For example:

```rust
#[unsafe(naked)]
unsafe extern "C" fn find_file_parameters() {
    asm!("push ebx", "push ecx", "push edx", "push eax", "call {}", "add esp, 4", "pop edx", "pop ecx", "pop ebx", "ret", sym find_file_impl, options(noreturn));
}

unsafe fn find_file_impl(a1: *const u8) -> u32 {}
```

In this case, the actual game code would pass the `a1` argument in `EAX`, and so in the unsafe(naked) function we hand
write assembly to `push` `EAX` to the stack, and then call our actual method, so that it can receive the argument in a
way that makes sense for Rust.

And you might be asking, what is the purpose of `push`ing and `pop`ing `EBX` and `ECX` as well?

Well the original function returns those two registers exactly the same as when it was entered, and to keep consistency
within the game state, we are good boys and do what the game expects :)

# Why in Rust?

Cause I think it's pretty neat :)
