# rustyboy
A toy gameboy emulator implemented in Rust

```
┌─────────────────────────────────────────────┐
│MainBoard                                    │
│ ┌────────────┐ ┌──────────────────────────┐ │
│ │  Cpu       │ │MemoryManagementUnit      │ │
│ │            ->│ ┌───────────┐            │ │
│ │            │ │ │ Cartridge │      ┌─┐   │ │
│ └────────────┘ │ ├───┬───────┘      │I│   │ │
│                │ │Apu│              │n│   │ │
│                │ ├───┤              │t│   │ │
│                │ │Gpu├─────────────►│e│   │ │
│                │ ├───┴──┐           │r│   │ │
│                │ │Joypad├──────────►│r│   │ │
│                │ ├──────┴────┐      │u│   │ │
│                │ │SerialCable├─────►│p│   │ │
│                │ ├─────┬─────┘      │t│   │ │
│                │ │Timer├───────────►│s│   │ │
│                │ └─────┘            └─┘   │ │
│                │ ┌────┐ ┌────┐ ┌────┐     │ │
│                │ │Hdma│ │Hram│ │Wram│     │ │
│                └─┴────┴─┴────┴─┴────┴─────┘ │
└─────────────────────────────────────────────┘
generated with asciiflow.com
```

References
https://gbdev.io/pandocs
https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
https://rgbds.gbdev.io/docs/v0.5.2/gbz80.7#RRC_r8
