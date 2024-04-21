<h1 align="center">libinspector</h1>
<p align="center">Multi-purpose Linux memory manipulation and interaction crate<p>
<p align="center">
    <img src="https://img.shields.io/badge/Built with Rust-grey?style=for-the-badge&logo=rust&color=%23B94700">
    <a target="_blank" href="https://github.com/standard3/libinspector/releases/">
        <img src="https://img.shields.io/github/v/release/standard3/libinspector?style=for-the-badge&color=%23DAA632&labelColor=%23B94700">
    </a>
    <img src="https://img.shields.io/github/license/standard3/libinspector?style=for-the-badge&color=%23DAA632&labelColor=%23B94700">
</p>

<br>

Supports Linux kernel >= 4.5

## Roadmap

**v0.1.0** - Introspection 1
- [ ] Find processes
- [ ] Find modules
- [ ] Find symbols
- [ ] Parse memory regions (pages informations, ...)
- [ ] Read / write memory

**v0.2.0** - Introspection 2 + Hooking
- [ ] Pattern or signature search
- [ ] Hooking / detouring functions

**v0.3.0** - Injection
- [ ] Memory injection / patching

**v0.4.0** - Assemble
- [ ] Assemble / disassemble code

**v0.5.0** - Debugging
- [ ] Debugging functions
- [ ] System inspection functions

## References

Similar projects :

- [scanmem](https://github.com/scanmem/scanmem)
- [rdbo/libmem](https://github.com/rdbo/libmem)
- [hax-rs/hax](https://github.com/hax-rs/hax)
- [Tommoa/rs-process-memory](https://github.com/Tommoa/rs-process-memory)
- [rmccrystal/memlib-rs](https://github.com/rmccrystal/memlib-rs)
- [etra0/memory-rs](https://github.com/etra0/memory-rs)
- [MrElectrify/memscan](https://github.com/MrElectrify/memscan)
