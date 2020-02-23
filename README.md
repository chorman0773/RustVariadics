# RustVariadics

`variadic!` 
This is a proc-macro crate that provides the ability to define variadic implementations of traits, with variadic functions
 and types planned. 
Note that these are not "truely" variadic. As rust itself does not posses variadics, 
 variadic implementations are expanded to a series of static-arity versions. 
This crate simply allows users to emulate variadics in some circumstances, without repeating code. 
 
## In-progress Features

* Variadic Generics
    * Generic Bounds
* Variadic Patterns
    * Type Patterns
    * Destructuring
    * `self...` (and `&self...`, and `&mut self...`, but not special stuff, yet)
* Pattern Expansions
    * Fold Expressions
    * `last` expansion
    * `sizeof...`
* Variadic self arguments
* Variadic Implementations

### Variadic Generics

The core functionality of this crate is to allow you to declare parameter packs, like T..., 
 in generic lists. Parameter Packs are, by default, expanded to packs of size 0 through 12. This can be changed,
 with an explicit size, such as `nonempty`, which expands to packs of size 1 through 12,
  or `size(<n>..[[=]m])`, which generates from size *n* to *m* (inclusively or exclusively),
  or from *n* to *n*+12 if m isn't specified.
  `size(<n>)` can also be used (which expands to a single pack of size *n*),
  though this is not generally used with a specific values.
  In this form, n can be a `sizeof...` expansion. 
  Only packs with size up to 255 will be generated due to implementation limits (and not wanting to kill rustc). 
  
When declaring a generic parameter pack, you can specify generic bounds as with regular rust. 
 By default this applies to **all** generics in the pack. Currently, it is possible to limit the application to only the last 
 bound in the set by using `last`. This would primarily be used with `?Sized`.
(For example `impl<T...: last ?Sized> MyTrait for (T...){}`).

Yes, nightly const generics will work with parameter packs. (Can't think of huge number of uses for it though). 

### Variadic Patterns

Whenever you declare something that looks like a parameter pack, but that depends on another pack, that is a variadic pattern. 
 Variadic Patterns have the same length as the dependent pack. This can be an associated constant, associated type,
 or some destructuring pattern.  

As an example, if you have a parameter `(t...): &(T...)`, then `t...` are the references to members of a tuple consiting of the expansion of `T...`. 

You can then access perform actions on each reference, with a variadic expansion. 
A variadic pattern does not have to depend directly on a parameter pack. Rather it can depend on a "Pack Expansion". 
This is detailed later, but an example is `(t...): (Box<T>...)`. 
Variadic Patterns always have the same size as the pack contained in the expansion. 
Variadic Patterns do not need to explicitly define the expansion it depends on (for example, if s is `(T...)`, 
 you can declare `let (t...) = s`, and t... is a variadic pattern dependent on `T...`). 

Variadic Patterns are themselves parameter packs (though with the shown special rules), and therefore can be expanded, 
 and used to declare other.

#### Reciever Patterns

As a special case, in variadic implementations for tuples, `self...` (and similarily `&self...` and `&mut self...`),
 are variadic patterns. 
 `self...` is considered to be a destructuring pattern applied to the actual reciever,
  as though it were `t...` declared as
  `let [mut] (t...) = self;`, where `self` is the actual reciever. 

This can be useful when implementing methods in traits,
 as it allows tuple implementations to simply call each of the contained versions method
 (and possibly package the results into a target tuple). 

Reciever Patterns are not yet supported for any reciever types other than `Self`, `&Self`, or `&mut Self`. 
All other Reciever types are planned. 

(Note, this syntax is subject to change. There has been indications of confusion with this syntax). 

### Pack Expansions

Pack Expansions allow parameter packs to be useful. A pack expansion consits of some construct,
 which contains 1 or more parameter packs (that are not otherwise expanded), followed by `...` (or which has a special form). 

There are 2 types of normal expansions:
* Expression Expansions (including Block Expansions)
* Type Expansions

