# Overview

The main UI of the application is a stack of multiple widgets.

```
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
░░┌─────────────────────┐░░░░
░░│ Scrollable Area     │░░░░
░░│  ┌──────────────────┴──┐░
░░│  │ Grid Widget         │░
░░│  │                     │░
░░│  │                     │░
░░│  │  ┌──────────────────┴──┐
░░└──┤  │ Status Label Widget │
░░░░░│  └──────────────────┬──┘
     └─────────────────────┘
```

The Grid widget is the main canvas, it's a grid of NxN characters, stored as
a `Vec<GridCell>` called `main_grid`. Only the visible area of the grid will
be rendered to the screen during `paint()` phase.

This visible area is controlled by the Scroll widget.

All the user's interaction as well as the application's data is handled by
the Grid widget.

The ApplicationState will be used to share data between widgets, not much is
being used for now, see the Staus Label widget for an example.

# Shapes list and the rendering process

To manage the list of drawing objects on the canvas, we use a `ShapeList`, its
a vector of `Shape`. Each `Shape` will define the `draw()` method, they're not
actually draw anything on the screen, but manipulate the `main_grid` instead.

To draw the shapes on the screen, we first iterate through the shape list and
call `draw()` method of each `is_preview()` shapes. At the end of this process,
we have the final character grid in `main_grid`, so we can render all of them
at once.

Use this approach, we get layer management for free(ish): Shapes that being
added to the list first will be added to the grid first, the later ones will
change the grid later, and we will know what to do with the overlapped lines.

To bring an object upfront or send them backward, we can just move them around
in the list.

To move or resize a shape, we just need to update their properties ande let the
`draw()` method handle the rest.

# Shapes and Tool

We use a `ToolManager` to manage which shape is currently being used. Each tool
implements the `ToolControl` trait, which defined the actions that will be called
for each mouse events:

```
                ┌──────────────┐
 MouseDown ────▷│              ├─────▷ start()
 MouseMove ────▷│ ToolManager  ├─────▷ draw()
   MouseUp ────▷│┌────────────┐├─────▷ end()
                └┤Active Tool ├┘
                 └────────────┘
```

For each tool, in the `start()` method, we append a new corresponding shape to
shape list, with the currest mouse position as the `start` position of that shape,
during the `draw()` method, mostly what we need to do is to capture the current
mouse position and update it as the `end` position of that shape. Do whatever you
want with the `end()` method.

See the following diagram for the logic flow between the `ToolManager` and the
`ShapeList`:

```
                ┌───────────────────┐        ┌───────────────────┐        ┌───────────────────┐
                │   Tool Manager    │        │    Shape List     │        │     Grid Data     │
                └─────────┬─────────┘        └─────────┬─────────┘        └─────────┬─────────┘
                          │                            │                            │
                          │  Create a preview          │                            │
                         ┌┴┐ shape                    ┌┴┐                           │
       Mouse Down        │ │─────────────────────────▶│ │                           │
                         └┬┘                          └┬┘                           │
                          │                            │                            │
 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┼ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│─ ─ ─ ─ ─
                          │                            │                            │
                         ┌┴┐       Get current shape  ┌┴┐                           │
       Mouse Move        │ │◀─────────────────────────│ │                           │
  (while drawing)        │ │                          │ │                           │
                         │ │─────────────────────────▶│ │                           │
                         └┬┘ Update shape             └┬┘                           │
                          │                            │                            │
 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┼ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│─ ─ ─ ─
                          │                            │                            │
                         ┌┴┐       Get current shape  ┌┴┐                           │
        Mouse Up         │ │◀─────────────────────────│ │                           │
 (while drawing)         │ │                          │ │                           │
                         │ │─────────────────────────▶│ │                           │
                         └┬┘ Mark as permanent        └┬┘                           │
                          │  shape                     │                            │
 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┼ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│─ ─ ─ ─ ─
                          │                            │                            │
                          │                           ┌┴┐                           │
                          │                       ┌───│ │                           │
                          │  Get shapes need redraw   │ │                           │
          Render          │                       └──▶│ │                           │
                          │                           │ │                           │
                          │                           │ │──────────────────────────▶│
                          │                           └┬┘ Put shape data            │
                          │                            │  to the grid               │
                          │                            │                            │
                          │                            │                            │
                          ▼                            ▼                            ▼
```

# Undo and Redo

When cell content changed, we call it an `Edit`. An `Edit` contains 3 fields,
the `index` of a cell that changed, the original content and the new content.
Normally, when the user perform an action, like drawing a shape, we will have 
multiple edits, and it called a `Version`.

``` 
Edit                     Version                                        
┌───┐     ┌───┐          ┌──────┬──────┬──────┬──────┬──────┬──────┬───┐
│ A │────▶│ B │          │ Edit │ Edit │ Edit │ Edit │ Edit │ Edit │...│
└───┘     └───┘          └──────┴──────┴──────┴──────┴──────┴──────┴───┘
  at <index>                                                            
```

When editing, the user can perform Undo or Redo by saving/restoring the edit
state based on the list of `Version`. We call this list a `History`, it also 
have an `index` pointer to tell what's the current history position.

```
History                                                                 
┌─────────┬─────────┬─────────┬─────────┬─────────┬───┐                 
│ Version │ Version │ Version │ Version │ Version │...│                 
└─────────┴─────────┴────🭯────┴─────────┴─────────┴───┘                 
                         │index                                         
```

In its normal state, the `History` object will have an `index` pointing to the
last `Version` element. When the user perform **Undo** action, we decrease the 
`index` pointer, and restore the editor state to the `History[index]` version. When
the user perform **Redo**, we increase the `index`, and apply the `History[index]` version
to the editor's state.

Keeping track of the undo history is pretty straightforward, for every action,
we build the `Version` object, which contains all of the `Edit`, and push it to 
the end of the `History` array.

```
Save a new Version to the end of History list                           
┌────┬────┬────┬────┐   ┌────┐                                          
│ V1 │ V2 │ V3 │ V4 │◀──┤ V5 │                                          
└────┴────┴────┴─🭯──┘   └────┘                                          
                 │index                                                 
```

One special case is when the user performed a few **undo** and the `index` is now
at the middle of the `History` array. Any new edit come after that will replace the 
`History` array at the point of `index`:

``` 
Save a new Version to the middle of History list                        
                      ┌────┐                                            
               ┌──────│ V5 │                                            
               │      └────┘                                            
┌────┬────┬────🭭────┐             ┌────┬────┬────┬────┐                 
│ V1 │ V2 │ V3 │ V4 │     ==>     │ V1 │ V2 │ V3 │ V5 │                 
└────┴────┴─🭯──┴────┘             └────┴────┴────┴─🭯──┘                 
            │index                                 │index               
```
