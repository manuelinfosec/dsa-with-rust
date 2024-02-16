use std::marker::PhantomData;

/// Beloved House
#[derive(Debug)]
struct House {
    floors: u32,
    rooms: u32,
    has_garage: bool,
}

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
impl BuilderState for HasRooms {} //
impl BuilderState for HasGarage {}
impl BuilderState for HasFloorsAndRooms {}
impl BuilderState for HasGarageAndFloors {}
impl BuilderState for HasGarageAndRooms {}
impl BuilderState for Ready {}

struct HouseBuilder<T: BuilderState> {
    _floors: u32,
    _rooms: u32,
    _has_garage: bool,
    _marker: PhantomData<T>,
}

impl HouseBuilder<Blueprint> {
    fn new() -> Self {
        Self {
            _floors: 0,
            _rooms: 0,
            _has_garage: false,
            _marker: PhantomData,
        }
    }

    fn floors(self, floors: u32) -> HouseBuilder<HasFloors> {
        HouseBuilder {
            _floors: floors,
            _rooms: self._rooms,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }

    fn rooms(self, rooms: u32) -> HouseBuilder<HasRooms> {
        HouseBuilder {
            _floors: self._floors,
            _rooms: rooms,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }

    fn with_garage(self) -> HouseBuilder<HasGarage> {
        HouseBuilder {
            _rooms: self._rooms,
            _floors: self._floors,
            _has_garage: true,
            _marker: PhantomData,
        }
    }
}

impl HouseBuilder<HasRooms> {
    fn floors(self, floors: u32) -> HouseBuilder<HasFloorsAndRooms> {
        HouseBuilder {
            _rooms: self._rooms,
            _floors: floors,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }
}

impl HouseBuilder<HasGarage> {
    fn floors(self, floors: u32) -> HouseBuilder<HasGarageAndFloors> {
        HouseBuilder {
            _floors: floors,
            _rooms: self._rooms,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }

    fn rooms(self, rooms: u32) -> HouseBuilder<HasGarageAndRooms> {
        HouseBuilder {
            _rooms: rooms,
            _floors: self._floors,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }
}

impl HouseBuilder<HasFloors> {
    fn rooms(self, rooms: u32) -> HouseBuilder<HasFloorsAndRooms> {
        HouseBuilder {
            _rooms: rooms,
            _floors: self._floors,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }

    fn with_garage(self) -> HouseBuilder<HasGarageAndFloors> {
        HouseBuilder {
            _rooms: self._rooms,
            _floors: self._floors,
            _has_garage: true,
            _marker: PhantomData,
        }
    }
}

impl HouseBuilder<HasGarageAndFloors> {
    fn rooms(self, rooms: u32) -> HouseBuilder<Ready> {
        HouseBuilder {
            _has_garage: true,
            _rooms: rooms,
            _floors: self._floors,
            _marker: PhantomData,
        }
    }
}

impl HouseBuilder<HasFloorsAndRooms> {
    fn with_garage(self) -> HouseBuilder<Ready> {
        HouseBuilder {
            _floors: self._floors,
            _rooms: self._rooms,
            _has_garage: self._has_garage,
            _marker: PhantomData,
        }
    }

    fn build(self) -> House {
        House {
            floors: self._floors,
            rooms: self._rooms,
            has_garage: self._has_garage,
        }
    }
}

impl HouseBuilder<HasGarageAndRooms> {
    fn floors(self, floors: u32) -> HouseBuilder<Ready> {
        HouseBuilder {
            _has_garage: true,
            _rooms: self._rooms,
            _floors: self._floors,
            _marker: PhantomData,
        }
    }

    fn build(self) -> House {
        House {
            floors: self._floors,
            rooms: self._rooms,
            has_garage: self._has_garage,
        }
    }
}

impl HouseBuilder<Ready> {
    fn build(self) -> House {
        House {
            floors: self._floors,
            rooms: self._rooms,
            has_garage: self._has_garage,
        }
    }
}

fn main() {
    let house = HouseBuilder::new().with_garage().floors(2).rooms(2).build();
    let house = HouseBuilder::new().rooms(2).floors(2).build();
    let hosue = HouseBuilder::new().floors(2).rooms(2).build();
    let house = HouseBuilder::new().floors(2).with_garage().rooms(2).build();

    println!("{house:#?}")
}
