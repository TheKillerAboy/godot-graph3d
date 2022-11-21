use gdnative::prelude::*;

mod graph3d;
mod execute;
mod lexer;
mod ast;
mod parse;

fn init(handle: InitHandle) {
    handle.add_class::<graph3d::Graph3D>();
}

godot_init!(init);
