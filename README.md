# rustyboy
A toy gameboy emulator implemented in Rust

```
┌─────────────────────────────────────────────┐
│MainBoard                                    │
│ ┌────────────┐ ┌──────────────────────────┐ │
│ │ThrottledCpu│ │MemoryManagementUnit      │ │
│ │ ┌───┐      │ │ ┌───────────┐            │ │
│ │ │Cpu│      │ │ │ Cartridge │      ┌─┐   │ │
│ └─┴───┴──────┘ │ ├───┬───────┘      │I│   │ │
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