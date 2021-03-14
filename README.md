# Sync
A research project about synchronizability and cosynchronizability

### Motivation

Equivalence classes in Outside theories of
[Avatar Extensions](https://advancedresearch.github.io/avatar-extensions/summary.html)
imposes weaker conditions than normalizability,
called *synchronizability* and *cosynchronizability*
(see [paper](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/synchronizability-and-cosynchronizability.pdf)).

This research project explores these properties using a common abstract in Rust.

### Introduction to Synchronizability and Cosynchronizability

In mathematics of sets, if `a = b` then we usually don't care about *how* they are equal.

In [Homotopy Type Theory](https://homotopytypetheory.org/),
objects can be equal in the sense that there exists a continuous map
from one object to another. These maps can have continuous maps
between maps and so on. This is very useful for theorem proving about topological spaces. Using this intuition, two objects `a` and `b`
are equal if there exists a continuous map between them,
but this does not imply higher order maps.

For example, on a torus, it is possible to create two loops which
can not be continuously transformed into each other.

In higher dimensional mathematics, specially in the theory of
[Avatar Extensions](https://advancedresearch.github.io/avatar-extensions/summary.html),
mathematical objects might not have a unique identity,
which is a bit like transforming objects using continuous maps.
However, instead of building this structure directly using maps,
it is beneficial to take a step back and think of a weaker kind of transformation.

When two objects are synchronized, it means that they both exist in a state
where they can be compared. With other words, the "time" which these
object exist is the same. So, when one object is brought to the "time"
of another object, they can be put in a relationship to each other,
while in different times this relationship has no meaning.

Synchronizability and Cosynchronizability are two properties that makes
it easy to compare objects of some kind by bringing them to the same "time"
in a systematic way.

- Synchronizability: An item that can be brought to the time of others
- Cosynchronizability: An item which time can bring other items to itself

This way of thinking is slightly more flexible and perhaps more applicable
than homotopy maps, which might help build intuition about hihgher dimensional mathematics.
