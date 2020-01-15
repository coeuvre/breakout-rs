use std::collections::VecDeque;

pub trait Index<T> {
    fn index(&self) -> usize;
    fn valid(&self, entry: &OccupiedEntry<T>) -> bool;
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GIndex {
    pub index: usize,
    pub generation: usize,
}

impl GIndex {
    pub fn new(index: usize, generation: usize) -> GIndex {
        GIndex { index, generation }
    }
}

impl<T> Index<T> for GIndex {
    fn index(&self) -> usize {
        self.index as usize
    }

    fn valid(&self, entry: &OccupiedEntry<T>) -> bool {
        self.generation == entry.generation
    }
}

impl<T> Index<T> for &GIndex {
    fn index(&self) -> usize {
        self.index as usize
    }

    fn valid(&self, entry: &OccupiedEntry<T>) -> bool {
        self.generation == entry.generation
    }
}

impl<T> Index<T> for usize {
    fn index(&self) -> usize {
        *self
    }

    fn valid(&self, _entry: &OccupiedEntry<T>) -> bool {
        true
    }
}

enum Entry<T> {
    Occupied(OccupiedEntry<T>),
    Vacant(VacantEntry),
}

impl<T> Entry<T> {
    fn occupied_entry_mut(&mut self) -> Option<&mut OccupiedEntry<T>> {
        match self {
            Entry::Occupied(entry) => Some(entry),
            _ => None,
        }
    }
}

pub struct OccupiedEntry<T> {
    pub generation: usize,
    pub value: T,
}

struct VacantEntry {
    pub generation: usize,
}

pub struct IndexVec<T> {
    entries: Vec<Entry<T>>,
    free: VecDeque<usize>,
}

impl<T> Default for IndexVec<T> {
    fn default() -> Self {
        IndexVec::new()
    }
}

impl<T> IndexVec<T> {
    pub fn new() -> IndexVec<T> {
        IndexVec {
            entries: Vec::new(),
            free: VecDeque::new(),
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.entries.len() {
            self.remove(i);
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn get<I: Index<T>>(&self, index: I) -> Option<&T> {
        if let Some(Entry::Occupied(entry)) = self.entries.get(index.index()) {
            if index.valid(entry) {
                return Some(&entry.value);
            }
        }

        None
    }

    pub fn get_mut<I: Index<T>>(&mut self, index: I) -> Option<&mut T> {
        if let Some(Entry::Occupied(entry)) = self.entries.get_mut(index.index() as usize) {
            if index.valid(entry) {
                return Some(&mut entry.value);
            }
        }

        None
    }

    pub fn get_two_mut<I1: Index<T>, I2: Index<T>>(
        &mut self,
        index_a: I1,
        index_b: I2,
    ) -> (Option<&mut T>, Option<&mut T>) {
        let mut a = None;
        let mut b = None;

        if index_a.index() < self.entries.len() {
            if let Entry::Occupied(entry) =
                unsafe { &mut *(self.entries.get_unchecked_mut(index_a.index()) as *mut Entry<T>) }
            {
                if index_a.valid(entry) {
                    a = Some(&mut entry.value);
                }
            }
        }

        if index_b.index() != index_a.index() && index_b.index() < self.entries.len() {
            if let Entry::Occupied(entry) =
                unsafe { &mut *(self.entries.get_unchecked_mut(index_b.index()) as *mut Entry<T>) }
            {
                if index_b.valid(entry) {
                    b = Some(&mut entry.value);
                }
            }
        }

        (a, b)
    }

    pub fn insert(&mut self, value: T) -> GIndex {
        if let Some(index) = self.free.pop_front() {
            let entry = &mut self.entries[index as usize];
            let generation = match entry {
                Entry::Vacant(entry) => entry.generation + 1,
                _ => unreachable!(),
            };
            *entry = Entry::Occupied(OccupiedEntry { generation, value });
            GIndex::new(index, generation)
        } else {
            self.entries.push(Entry::Occupied(OccupiedEntry {
                generation: 0,
                value,
            }));
            GIndex::new(self.entries.len() - 1, 0)
        }
    }

    pub fn remove<I: Index<T>>(&mut self, index: I) -> Option<T> {
        if let Some(entry) = self.entries.get_mut(index.index()) {
            let mut new_entry = None;
            if let Entry::Occupied(occupied_entry) = entry {
                if index.valid(occupied_entry) {
                    new_entry = Some(Entry::Vacant(VacantEntry {
                        generation: occupied_entry.generation,
                    }));
                }
            }

            if let Some(mut new_entry) = new_entry {
                std::mem::swap(entry, &mut new_entry);
                self.free.push_back(index.index());

                match new_entry {
                    Entry::Occupied(entry) => {
                        return Some(entry.value);
                    }
                    _ => unreachable!(),
                }
            }
        }

        None
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            index_vec: self,
            i: 0,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            index_vec: self,
            i: 0,
        }
    }

    pub fn iter_two_mut(&mut self) -> IterTwoMut<T> {
        IterTwoMut {
            index_vec: self,
            i: 0,
            j: 1,
        }
    }
}

pub struct Iter<'a, T> {
    index_vec: &'a IndexVec<T>,
    i: usize,
}

impl<'a, T> Iter<'a, T> {
    pub fn with_index(self) -> IterWithIndex<'a, T> {
        IterWithIndex {
            index_vec: self.index_vec,
            i: self.i,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.index_vec.entries.len() {
            let index = self.i;
            self.i += 1;
            if let Some(Entry::Occupied(entry)) = self.index_vec.entries.get(index) {
                return Some(&entry.value);
            }
        }

        None
    }
}

pub struct IterWithIndex<'a, T> {
    index_vec: &'a IndexVec<T>,
    i: usize,
}

impl<'a, T> Iterator for IterWithIndex<'a, T> {
    type Item = (GIndex, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.index_vec.entries.len() {
            let index = self.i;
            self.i += 1;
            if let Some(Entry::Occupied(entry)) = self.index_vec.entries.get(index) {
                return Some((GIndex::new(index, entry.generation), &entry.value));
            }
        }

        None
    }
}

pub struct IterMut<'a, T> {
    index_vec: &'a mut IndexVec<T>,
    i: usize,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.index_vec.entries.len() {
            let index = self.i;
            self.i += 1;
            let entry =
                unsafe { &mut *(self.index_vec.entries.get_unchecked_mut(index) as *mut Entry<T>) }
                    .occupied_entry_mut()
                    .map(|entry| &mut entry.value);
            if entry.is_some() {
                return entry;
            }
        }

        None
    }
}

pub struct IterTwoMut<'a, T> {
    index_vec: &'a mut IndexVec<T>,
    i: usize,
    j: usize,
}

impl<'a, T> Iterator for IterTwoMut<'a, T> {
    type Item = (&'a mut T, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.index_vec.entries.len() {
            let i = self.i;
            if let Entry::Occupied(entry_a) =
                unsafe { &mut *(self.index_vec.entries.get_unchecked_mut(i) as *mut Entry<T>) }
            {
                while self.j < self.index_vec.entries.len() {
                    let j = self.j;

                    self.j += 1;
                    if self.j > self.index_vec.entries.len() {
                        self.i += 1;
                        self.j = self.i + 1;
                    }

                    if i == j {
                        continue;
                    }

                    if let Entry::Occupied(entry_b) = unsafe {
                        &mut *(self.index_vec.entries.get_unchecked_mut(j) as *mut Entry<T>)
                    } {
                        return Some((&mut entry_a.value, &mut entry_b.value));
                    }
                }
            }
        }

        None
    }
}
