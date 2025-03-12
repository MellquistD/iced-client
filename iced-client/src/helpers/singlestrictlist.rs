pub struct SingleStrictSelectionList<T> {
    items: Vec<T>,
    selected_index: usize,
}

impl<T: PartialEq> SingleStrictSelectionList<T> {
    /// Creates a new SingleStrictSelectionList with the given items and initially selected index.
    ///
    /// # Panics
    /// Panics if selected_index is out of bounds (>= items.len()) or if items is empty.
    pub fn new(items: Vec<T>, selected_index: usize) -> Self {
        assert!(!items.is_empty(), "Items cannot be empty");
        assert!(selected_index < items.len(), "Selected index out of bounds");

        Self {
            items,
            selected_index,
        }
    }

    /// Toggles selection to the specified index.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn toggle_by_index(&mut self, index: usize) {
        assert!(index < self.items.len(), "Index out of bounds");
        self.selected_index = index;
    }

    pub fn toggle(&mut self, item: &T) {
        let idx = self
            .items
            .iter()
            .position(|x| x == item)
            .expect("Item not found in list");
        self.toggle_by_index(idx);
    }

    /// Returns a reference to the currently selected item.
    pub fn get_selected(&self) -> &T {
        &self.items[self.selected_index]
    }

    /// Returns a mutable reference to the currently selected item.
    pub fn get_selected_mut(&mut self) -> &mut T {
        &mut self.items[self.selected_index]
    }

    /// Returns the index of the currently selected item.
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Returns the number of items in the list.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if the list contains no items (though this should never happen
    /// after construction due to the validation in new()).
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Consumes the list and returns an iterator over its items.
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.items.into_iter()
    }

    /// Returns an iterator over references to all items.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    /// Returns an iterator over mutable references to all items.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut()
    }

    /// Returns a reference to the item at the specified index.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn get(&self, index: usize) -> &T {
        &self.items[index]
    }

    /// Returns a mutable reference to the item at the specified index.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn get_mut(&mut self, index: usize) -> &mut T {
        &mut self.items[index]
    }

    /// Adds an item to the end of the list.
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Removes the item at the specified index.
    ///
    /// # Panics
    /// Panics if:
    /// - The index is out of bounds
    /// - This would remove the last item (list must always contain at least one item)
    pub fn remove(&mut self, index: usize) -> T {
        assert!(self.items.len() > 1, "Cannot remove the last item");
        assert!(index < self.items.len(), "Index out of bounds");

        let removed = self.items.remove(index);

        // Adjust selected_index if needed
        if index == self.selected_index {
            // If we removed the selected item, select the first item
            self.selected_index = 0;
        } else if index < self.selected_index {
            // If we removed an item before the selected one, adjust the index
            self.selected_index -= 1;
        }

        removed
    }

    /// Returns true if the specified index is currently selected.
    pub fn is_selected(&self, index: usize) -> bool {
        index == self.selected_index
    }
}

impl<T: Clone> SingleStrictSelectionList<T> {
    /// Returns a clone of the currently selected item.
    pub fn get_selected_cloned(&self) -> T {
        self.items[self.selected_index].clone()
    }
}

// Implement useful traits
impl<T> std::ops::Index<usize> for SingleStrictSelectionList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T> std::ops::IndexMut<usize> for SingleStrictSelectionList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}
