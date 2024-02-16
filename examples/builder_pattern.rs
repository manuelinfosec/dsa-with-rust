use std::marker::PhantomData;

/// The states representing the status of the construction
struct Blueprint;
struct HasFloors;
struct HasRooms;
struct HasGarage;
struct HasFloorsAndRooms;
struct HasGarageAndRooms;
struct HasGarageAndFloors;
struct Ready;

/// A trait that any struct that wants to be considered as a state needs to implement
trait BuilderState {}

impl BuilderState for Blueprint {}
impl BuilderState for HasFloors {}
impl BuilderState for HasRooms {}
impl BuilderState for HasGarage {}
impl BuilderState for HasFloorsAndRooms {}
impl BuilderState for HasGarageAndFloors {}
impl BuilderState for HasGarageAndRooms {}
impl BuilderState for Ready {}

/// Beloved House
#[derive(Debug)]
struct House {
    floors: u32,
    rooms: u32,
    has_garage: bool,
}

/// The builder pattern for constructing houses with various configurations
struct HouseBuilder<T: BuilderState> {
    _floors: u32,
    _rooms: u32,
    _has_garage: bool,
    _marker: PhantomData<T>, // Indicates that `T` is part of the type but doesn't contribute to data
}

impl HouseBuilder<Blueprint> {
    /// Creates a new HouseBuilder in the initial Blueprint state
    fn new() -> Self {
        Self {
            _floors: 0,
            _rooms: 0,
            _has_garage: false,
            _marker: PhantomData, // No state has been set initially
        }
    }

    /// Sets the number of floors and transitions to HasFloors state
    fn floors(self, floors: u32) -> HouseBuilder<HasFloors> {
        HouseBuilder {
            _floors: floors,
            _rooms: self._rooms,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }

    /// Sets the number of rooms and transitions to HasRooms state
    fn rooms(self, rooms: u32) -> HouseBuilder<HasRooms> {
        HouseBuilder {
            _floors: self._floors,
            _rooms: rooms,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }

    /// Sets the garage flag to true and transitions to HasGarage state
    fn with_garage(self) -> HouseBuilder<HasGarage> {
        HouseBuilder {
            _rooms: self._rooms,
            _floors: self._floors,
            _has_garage: true,
            _marker: PhantomData,
        }
    }
}

// Transitions and methods for HasFloors state
impl HouseBuilder<HasFloors> {
    /// Sets the number of rooms and transitions to HasFloorsAndRooms state
    fn rooms(self, rooms: u32) -> HouseBuilder<HasFloorsAndRooms> {
        HouseBuilder {
            _rooms: rooms,
            _floors: self._floors,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }

    /// Sets the garage flag to true and transitions to HasGarageAndFloors state
    fn with_garage(self) -> HouseBuilder<HasGarageAndFloors> {
        HouseBuilder {
            _rooms: self._rooms,
            _floors: self._floors,
            _has_garage: true,
            _marker: PhantomData,
        }
    }
}

// Transitions and methods for HasRooms state
impl HouseBuilder<HasRooms> {
    /// Sets the number of rooms and transtions to the HasFloorsAndRooms state
    fn floors(self, floors: u32) -> HouseBuilder<HasFloorsAndRooms> {
        HouseBuilder {
            _floors: floors,
            _rooms: self._rooms,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }

    fn with_garage(self) -> HouseBuilder<HasGarageAndRooms> {
        HouseBuilder {
            _floors: self._floors,
            _rooms: self._rooms,
            _has_garage: true,
            _marker: PhantomData,
        }
    }
}

// Transitions and methods for HasGarage state
impl HouseBuilder<HasGarage> {
    /// Sets the number of floors and transitions to HasGarageAndFloors state
    fn floors(self, floors: u32) -> HouseBuilder<HasGarageAndFloors> {
        HouseBuilder {
            _floors: floors,
            _rooms: self._rooms,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }

    /// Sets the number of rooms and transitions to HasGarageAndRooms state
    fn rooms(self, rooms: u32) -> HouseBuilder<HasGarageAndRooms> {
        HouseBuilder {
            _rooms: rooms,
            _floors: self._floors,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }
}

// Transitions and methods for HasFloorsAndRooms state
impl HouseBuilder<HasFloorsAndRooms> {
    /// Sets the garage flag to true and transitions to Ready state
    fn with_garage(self) -> HouseBuilder<Ready> {
        HouseBuilder {
            _floors: self._floors,
            _rooms: self._rooms,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }

    /// Constructs the House object with the provided configuration
    fn build(self) -> House {
        House {
            floors: self._floors,
            rooms: self._rooms,
            has_garage: self._has_garage,
        }
    }
}

// Transitions and methods for HasGarageAndFloors state
impl HouseBuilder<HasGarageAndFloors> {
    /// Sets the number of rooms and transitions to Ready state
    fn rooms(self, rooms: u32) -> HouseBuilder<Ready> {
        HouseBuilder {
            _has_garage: true,
            _rooms: rooms,
            _floors: self._floors,
            _marker: PhantomData,
        }
    }
}

// Transitions and methods for HasGarageAndRooms state
impl HouseBuilder<HasGarageAndRooms> {
    /// Sets the number of floors and transitions to Ready state
    fn floors(self, floors: u32) -> HouseBuilder<Ready> {
        HouseBuilder {
            _has_garage: true,
            _rooms: self._rooms,
            _floors: self._floors,
            _marker: PhantomData,
        }
    }

    /// Constructs the House object with the provided configuration
    fn build(self) -> House {
        House {
            floors: self._floors,
            rooms: self._rooms,
            has_garage: self._has_garage,
        }
    }
}

// Final Ready state with build method
impl HouseBuilder<Ready> {
    /// Constructs the House object with the provided configuration
    fn build(self) -> House {
        House {
            floors: self._floors,
            rooms: self._rooms,
            has_garage: self._has_garage,
        }
    }
}

fn main() {
    // Example usage
    let house1 = HouseBuilder::new().with_garage().floors(2).rooms(2).build();
    let house2 = HouseBuilder::new().rooms(2).floors(2).build();
    let house3 = HouseBuilder::new().floors(2).rooms(2).build();
    let house4 = HouseBuilder::new().floors(2).with_garage().rooms(2).build();

    // Print out the constructed houses
    println!("{:#?}", house1);
    println!("{:#?}", house2);
    println!("{:#?}", house3);
    println!("{:#?}", house4);
}
