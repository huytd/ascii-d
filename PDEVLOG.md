# Apr 06, 2023

## Eraser Advanced

#### Resize [-/=]

Idea: besides the default size eraser, it will have 3 other size: small(s: 3x3), medium(m: 5x5), large(l: 9x9)

Solution:

```rust
// > src/tools/mod.rs > impl ToolControl
// add this interface, which means it will be apply for all tools

fn resize(&mut self, option: ResizeOption) {
    self.available_tools[self.current].resize(option);
}

// > src/widgets/grid.rs > impl Widget
// only apply for EraserTool at the moment
// -/= instead of -/+

if win_data.mode == DrawingTools::Eraser {
    match keycode {
        Code::Minus => {
            self.tool_manager.resize(ResizeOption::Decrease);
        }
        Code::Equal => {
            self.tool_manager.resize(ResizeOption::Increase);
        }
        _ => {}
    }
}

// define ToolsSize
pub enum ToolsSize {
    Default = 0,
    Small = 1 << 0,
    Medium = 1 << 1,
    Large = 1 << 2,
}

// > src/tools/eraser.rs
// two fn and an algo function for removing k-neighbors chars in 1D vec
pub fn increase_size {}
pub fn decrease_size {}

```

Progress:

- [x] users also can use `-` to decrease size or `=` to increase size of eraser
- [ ] when use hover the eraser button in the ui, a popup will open with [s,m,l]

Problems:

- [ ] still don't know how to trigger a hover or a popup
- [ ] apply `resize` interface for all tools seems ...
- [ ] the history vec seems not right after erasing

#### Shape-based

Idea: besides erasing cell by cell, user can also hold `Ctrl` and click to remove the whole shape. each shape wrapped around `number` will be erased just with a single `Ctrl`+`Click`

```

            ┌────────────────[8]──────────────────┐
            │                                     │
            │                                     │
            │                                    [7]
            │                                     │
            │                                     │
           [9]                                    │
            │                    ┌────────────────┴────[5]──────────────┐
            │                    │                                      │
            │                    │                                      │
            │                    │                                      │
  ┌─────────▼─[1]──────┐         │                                      │
  │                    │         │                                      │
  │                    │         │                                     [5]
  │                    │         │                                      │
  │                    │         │                                      │
 [1]                  [1]        │                     ┌───────────[6]──┼───────┐
  │                    │        [5]                    │                │       │
  │                    │         │                     │                │       │
  │                    │         │                     │                │       │
  │                    │         │                     │                │       │
  │                    │         │                     │                │       │
  └────────[1]─────────┘         │                    [6]               │      [6]
            │                    │                     │                │       │
            │                    │                     │                │       │
            │                    │                     │                │       │
           [2]                   └───────[5]──────▲────┼────────────────┘       │
            │                                     │    │                        │
            │                                     │    │                        │
            │                                    [4]   └───────────[6]──────────┘
            │                                     │
            │                                     │
            └───────────────[3]───────────────────┘

```

Solution:

I first implemented this feature based on `HISTORY_MANAGER`, trying to erase the shape that each history version saved, following the logic:

- get current cursor position: `pos`
- loop to `history.versions` vector
- loop to each `version.edits` vector
- check if `edit.index == pos`, return the whole `index` in that `edits`
- update `grid_list[index] = CHAR_SPACE`
- `HISTORY_MANAGER.save_version()`

Although, I believe this shouldn't be the right way to implement this. shouldn't mess up with the `HISTORY_MANAGER`

Progress:

- [x] Able to erase shapes, but not really :anguished:
- [x] Able to undo/redo after erasing, but not really :anguished:
- [ ] Try another method, but a bit afraid of the algorithm :anguished:

Problems:

- [ ] Overlapping shaped after being erased will miss some strokes

```
  ┌────────────────┐
  │                │
  │                │
  │                │
  │                │                   ┌─────────── ──────┐
  │     ┌──────────┼──────┐            │                  │
  │     │          │      │─────────▶  │    ┌────── ───┐  │
  │     │   ┌──────┼──┐   │            │    │          │  │
  │     │   │      │  │   │            │    │          │  │
  │     │   │      │  │   │            │    │          │  │
  └─────┼───┼──────┘  │   │                            │  │
        │   │         │   │            │    │          │  │
        │   │         │   │            │    └──────────┘  │
        │   └─────────┘   │            │                  │
        └─────────────────┘            └──────────────────┘

```

- [x] After user select the whole area and remove that with Selection tool, everything inside that area will be treated as one shape (because `edit.index` always `== pos` at this point) <--- I'm trying to bypass with by specifing the tool stored the version and ignore the tool == Selection

```rust
// > src/data/history.rs
pub struct Edit {
    index: usize,
    from: char,
    to: char,
    tool: DrawingTools, // <------ this code sucks !!!
}

```

- [ ] When we copy/paste a combination of shapes, that will be grouped too :hear_no_evil:

- [ ] If we still keeping this method, the code need to be compatible with another method by just one change
