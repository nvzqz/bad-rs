/// A type alias to [`!` (never)][never] that works in places `!` doesn't
/// currently in stable Rust.
///
/// # Examples
///
/// This is a way to indirectly refer to `!` in places where using it
/// directly isn't allowed. Such as simply making an alias to `!` via normal
/// means:
///
/// ```compile_fail
/// type Never = !;
/// ```
///
/// However, with this alias, you can make another alias to `!` indirectly:
///
/// ```
/// type Never = bad::Never;
/// ```
///
/// ## Return type
///
/// Just like `!` can already, `Never` can be used as a function return type:
///
/// ```
/// fn error() -> bad::Never {
///     panic!();
/// }
///
/// let error_fn: fn() -> ! = error;
/// ```
///
/// ## Input type
///
/// Currently, one can't use `!` as a function input type:
///
/// ```compile_fail
/// fn forward(never: !) -> ! {
///     never
/// }
/// ```
///
/// The same goes for expressing the function type:
///
/// ```compile_fail
/// type F = fn(!) -> !;
/// ```
///
/// By using `Never` in place of `!`, the function compiles:
///
/// ```
/// fn forward(never: bad::Never) -> ! {
///     never
/// }
///
/// let forward_fn: fn(bad::Never) -> ! = forward;
/// ```
///
/// ## Trait `impl`s on `!`
///
/// Currently, one can't `impl` custom traits directly on `!`:
///
/// ```compile_fail
/// trait NeverType {}
///
/// impl NeverType for ! {}
/// ```
///
/// By using `Never` in place of `!`, the `impl` works:
///
/// ```
/// # trait NeverType {}
/// impl NeverType for bad::Never {}
/// ```
///
/// However, this isn't of much use since `!` turns into `()` in the context
/// of trait bounds.
///
/// ## Array Item Type
///
/// Currently, one can't use `!` as the item type of an array:
///
/// ```compile_fail
/// let array: [!; 0] = [];
/// ```
///
/// The same for slices:
///
/// ```compile_fail
/// let slice: &[!] = &[];
/// ```
///
/// By using `Never` in place of `!` the above works:
///
/// ```
/// let array: [bad::Never; 0] = [];
/// let slice: &[bad::Never] = &[];
/// ```
///
/// [never]: https://doc.rust-lang.org/std/primitive.never.html
pub type Never = <F as HasOutput>::Output;

// Declared public to prevent errors when externally used. This is locally
// guarded against by denying `private_in_public` everywhere. However, it should
// not be actually publicly exposed. This is locally guarded against by denying
// `missing_docs` everywhere.
pub trait HasOutput {
    type Output;
}

impl<O> HasOutput for fn() -> O {
    type Output = O;
}

type F = fn() -> !;
