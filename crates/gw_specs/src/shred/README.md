Source = https://github.com/amethyst/shred
This is a modified copy of the Shred library.

Source = https://github.com/amethyst/shred
Version = 0.14.1

Modifications

- Replaced TrustCell {Ref, RefMut} with AtomicRefCell {AtomicRef, AtomicRefMut}.
- Split World into Resources and World.
  - World has the SystemData and System stuff, resources is just the HashMap.
