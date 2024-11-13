/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputCellId(u64);

/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

pub enum Cell<'a, T> {
    Input(InputCell<T>),
    Compute(ComputeCell<'a, T>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InputCell<T> {
    id: InputCellId,
    value: T,
}

type BoxedComputeFunc<'a, T> = Box<dyn Fn(&[T]) -> T + 'a>;

pub struct ComputeCell<'a, T> {
    id: ComputeCellId,
    value: T,
    dependencies: Vec<CellId>,
    compute_func: BoxedComputeFunc<'a, T>,
    callbacks: Vec<Callback<'a, T>>,
}

type BoxedCallbackFunc<'a, T> = Box<dyn FnMut(T) + 'a>;

struct Callback<'a, T> {
    id: CallbackId,
    callback_func: BoxedCallbackFunc<'a, T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    // Just so that the compiler doesn't complain about an unused type parameter.
    // You probably want to delete this field.
    // dummy: ::std::marker::PhantomData<T>,
    cells: Vec<Cell<'a, T>>,

    input_id: u64,
    compute_id: u64,
    callback_id: u64,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            input_id: 0,
            compute_id: 0,
            callback_id: 0,
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        self.input_id += 1;
        let id = InputCellId(self.input_id);
        let cell = Cell::Input(InputCell { id, value: initial });
        self.cells.push(cell);
        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let dependencie_cells = dependencies
            .iter()
            .map(|id| {
                self.cells
                    .iter()
                    .find(|cell| match (cell, id) {
                        (Cell::Input(cell), CellId::Input(id)) => id == &cell.id,
                        (Cell::Compute(cell), CellId::Compute(id)) => id == &cell.id,
                        _ => false,
                    })
                    .ok_or(*id)
            })
            .collect::<Result<Vec<&Cell<T>>, CellId>>()?;

        let value = compute_func(
            &dependencie_cells
                .iter()
                .map(|cell| match cell {
                    Cell::Input(cell) => cell.value,
                    Cell::Compute(cell) => cell.value,
                })
                .collect::<Vec<_>>(),
        );
        self.compute_id += 1;
        let id = ComputeCellId(self.compute_id);
        let cell = Cell::Compute(ComputeCell {
            id,
            value,
            dependencies: dependencies.to_vec(),
            compute_func: Box::new(compute_func),
            callbacks: Vec::new(),
        });
        self.cells.push(cell);

        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        // todo!("Get the value of the cell whose id is {id:?}")
        self.cells
            .iter()
            .find(|cell| match (cell, id) {
                (Cell::Input(cell), CellId::Input(id)) => id == cell.id,
                (Cell::Compute(cell), CellId::Compute(id)) => id == cell.id,
                _ => false,
            })
            .map(|cell| match cell {
                Cell::Input(cell) => cell.value,
                Cell::Compute(cell) => cell.value,
            })
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        let updated = self
            .cells
            .iter_mut()
            .find_map(|cell| match cell {
                Cell::Input(cell) if id == cell.id => Some(cell),
                _ => None,
            })
            .map(|cell| cell.value = new_value)
            .is_some();

        let mut updated_cell_ids = if updated {
            vec![CellId::Input(id)]
        } else {
            Vec::new()
        };

        use std::collections::HashSet;

        let mut compute_cell_ids = HashSet::new();
        while !updated_cell_ids.is_empty() {
            let new_compute_cell_values = self
                .cells
                .iter()
                .enumerate()
                .filter_map(|(i, cell)| match cell {
                    Cell::Compute(cell) => {
                        if updated_cell_ids
                            .iter()
                            .any(|id| cell.dependencies.contains(id))
                        {
                            Some((i, cell))
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .map(|(i, cell)| {
                    let dependencie_values = cell
                        .dependencies
                        .iter()
                        .map(|id| {
                            self.cells
                                .iter()
                                .find_map(|cell| match (cell, id) {
                                    (Cell::Input(cell), CellId::Input(id)) if id == &cell.id => {
                                        Some(cell.value)
                                    }
                                    (Cell::Compute(cell), CellId::Compute(id))
                                        if id == &cell.id =>
                                    {
                                        Some(cell.value)
                                    }
                                    _ => None,
                                })
                                .unwrap()
                        })
                        .collect::<Vec<_>>();

                    (i, (*cell.compute_func)(&dependencie_values))
                })
                .collect::<Vec<_>>();

            updated_cell_ids = new_compute_cell_values
                .into_iter()
                .filter_map(|(i, new_value)| match &mut self.cells[i] {
                    Cell::Compute(cell) => {
                        if cell.value != new_value {
                            cell.value = new_value;
                            compute_cell_ids.insert(cell.id);
                            Some(CellId::Compute(cell.id))
                        } else {
                            None
                        }
                    }

                    _ => unreachable!(),
                })
                .collect();
        }

        compute_cell_ids.into_iter().for_each(|id| {
            let cell = self
                .cells
                .iter_mut()
                .find_map(|cell| match cell {
                    Cell::Compute(cell) if id == cell.id => Some(cell),
                    _ => None,
                })
                .unwrap();
            cell.callbacks
                .iter_mut()
                .for_each(|cb| (*cb.callback_func)(cell.value));
        });

        updated
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        let cell = self.cells.iter_mut().find_map(|cell| match cell {
            Cell::Compute(cell) if id == cell.id => Some(cell),
            _ => None,
        })?;

        self.callback_id += 1;
        let id = CallbackId(self.callback_id);
        cell.callbacks.push(Callback {
            id,
            callback_func: Box::new(callback),
        });

        Some(id)
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        // todo!(
        //     "Remove the callback identified by the CallbackId {callback:?} from the cell {cell:?}"
        // )

        let cell = self
            .cells
            .iter_mut()
            .find_map(|c| match c {
                Cell::Compute(c) if cell == c.id => Some(c),
                _ => None,
            })
            .ok_or(RemoveCallbackError::NonexistentCell)?;

        cell.callbacks
            .iter()
            .position(|cb| callback == cb.id)
            .map(|index| {
                cell.callbacks.remove(index);
            })
            .ok_or(RemoveCallbackError::NonexistentCallback)
    }
}
