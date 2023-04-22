// Array struct
pub struct Array<T: Default> {
    pub data: Vec<Vec<T>>,
    rows: usize,
    columns: usize,
}

// Implementations on Array struct
impl<T: Default> Array<T> {
    // Creates a new Array struct with given items ( multi -dimensional vector ), rows and columns.
    // If enough items are not passed for each row, default values of that type are added.
    pub fn new(items: Vec<Vec<T>>, rows: usize, columns: usize) -> Array<T>
    where
        T: Clone,
    {
        // Cloning is not possible if T is not clonable.
        let mut data = items.clone();

        // If enough items are not passed in each row, add default values of that type.
        for i in 0..items.len() {
            if items[i].len() < columns {
                for _j in 0..columns - items[i].len() {
                    data[i].push(T::default());
                }
            }
        }

        // Create the Array struct instance
        let array = Array {
            data,
            rows,
            columns,
        };

        return array;
    }

    pub fn rows(&self) -> usize {
        return self.rows;
    }

    pub fn columns(&self) -> usize {
        return self.columns;
    }

    // Returns the dimensions of the array.
    pub fn dims(&self) -> (usize, usize) {
        return (self.rows, self.columns);
    }
}

// Returns an Array with given multi-dimensional vector.
// Required maximum number of items for each row to add default values if values are not sufficient.
pub fn init<T: Default>(arr: Vec<Vec<T>>, max_items_in_a_row: usize) -> Array<T>
where
    T: Clone,
{
    let rows = arr.len();
    let array = Array::new(arr, rows, max_items_in_a_row);
    return array;
}

// Returns an Array with all values initialised to the given single value.
pub fn init_all_with<T: Default>(value: T, rows: usize, columns: usize) -> Array<T>
where
    T: Clone,
{
    let mut arr = Vec::with_capacity(rows);
    for _i in 0..rows {
        let mut row = Vec::with_capacity(columns);
        for _j in 0..columns {
            row.push(value.clone());
        }
        arr.push(row);
    }
    return Array::new(arr, rows, columns);
}
