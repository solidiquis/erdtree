use crate::{
    context::Context,
    tree::Tree,
};
use std::marker::PhantomData;

pub struct Regular;
pub struct Inverted;
pub struct Flat;

pub struct Engine<T> {
    ctx: Context,
    //tree: Tree<T>,
    layout: PhantomData<T>
}

impl<T> Engine<T> {
    pub fn new(ctx: Context) -> Self {
        Self { ctx, layout: PhantomData }
    }
}
